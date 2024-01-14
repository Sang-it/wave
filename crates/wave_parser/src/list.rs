use wave_allocator::Vec;
use wave_ast::ast::FormalParameter;
use wave_diagnostics::Result;
use wave_lexer::Kind;

use crate::Parser;

pub trait SeparatedList<'a>: Sized {
    fn new(p: &Parser<'a>) -> Self;

    fn parse(p: &mut Parser<'a>) -> Result<Self> {
        let mut list = Self::new(p);
        list.parse_list(p)?;
        Ok(list)
    }

    /// Open element, e.g.. `{` `[` `(`
    fn open(&self) -> Kind;

    /// Close element, e.g.. `}` `]` `)`
    fn close(&self) -> Kind;

    /// Separator element, e.g. `,`
    fn separator(&self) -> Kind {
        Kind::Comma
    }

    fn parse_element(&mut self, p: &mut Parser<'a>) -> Result<()>;

    /// Main entry point, parse the list
    fn parse_list(&mut self, p: &mut Parser<'a>) -> Result<()> {
        p.expect(self.open())?;

        let mut first = true;

        while !p.at(self.close()) && !p.at(Kind::Eof) {
            if first {
                first = false;
            } else {
                p.expect(self.separator())?;
                if p.at(self.close()) {
                    break;
                }
            }

            self.parse_element(p)?;
        }

        p.expect(self.close())?;
        Ok(())
    }
}

pub struct FormalParameterList<'a> {
    pub elements: Vec<'a, FormalParameter<'a>>,
}

impl<'a> SeparatedList<'a> for FormalParameterList<'a> {
    fn new(p: &Parser<'a>) -> Self {
        Self {
            elements: p.ast.new_vec(),
        }
    }

    fn open(&self) -> Kind {
        Kind::LParen
    }

    fn close(&self) -> Kind {
        Kind::RParen
    }

    fn parse_element(&mut self, p: &mut Parser<'a>) -> Result<()> {
        let span = p.start_span();

        match p.cur_kind() {
            _ => {
                let pattern = p.parse_binding_pattern()?;
                let formal_parameter = p.ast.formal_parameter(p.end_span(span), pattern);
                self.elements.push(formal_parameter);
            }
        }

        Ok(())
    }
}
