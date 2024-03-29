use wave_allocator::Vec;
use wave_ast::ast::{
    Argument, ArrayExpressionElement, ClassElement, Expression, FormalParameter,
    ImportDeclarationSpecifier,
};
use wave_diagnostics::Result;
use wave_lexer::Kind;
use wave_span::Span;

use crate::Parser;

pub trait NormalList<'a> {
    /// Open element, e.g.. `{` `[` `(`
    fn open(&self) -> Kind;

    /// Close element, e.g.. `}` `]` `)`
    fn close(&self) -> Kind;

    fn parse_element(&mut self, p: &mut Parser<'a>) -> Result<()>;

    /// Main entry point, parse the list
    fn parse(&mut self, p: &mut Parser<'a>) -> Result<()> {
        p.expect(self.open())?;
        while !p.at(self.close()) && !p.at(Kind::Eof) {
            self.parse_element(p)?;
        }
        p.expect(self.close())?;
        Ok(())
    }
}

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

        let pattern = p.parse_binding_pattern()?;
        let formal_parameter = p.ast.formal_parameter(p.end_span(span), pattern);
        self.elements.push(formal_parameter);

        Ok(())
    }
}

pub struct SequenceExpressionList<'a> {
    pub elements: Vec<'a, Expression<'a>>,
}

impl<'a> SeparatedList<'a> for SequenceExpressionList<'a> {
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

    // read everything as expression and map to it to either
    // ParenthesizedExpression or ArrowFormalParameters later
    fn parse_element(&mut self, p: &mut Parser<'a>) -> Result<()> {
        let element = p.parse_assignment_expression_base()?;
        self.elements.push(element);
        Ok(())
    }
}

pub struct ArrayExpressionList<'a> {
    pub elements: Vec<'a, ArrayExpressionElement<'a>>,
    pub trailing_comma: Option<Span>,
}

impl<'a> SeparatedList<'a> for ArrayExpressionList<'a> {
    fn new(p: &Parser<'a>) -> Self {
        Self {
            elements: p.ast.new_vec(),
            trailing_comma: None,
        }
    }

    fn open(&self) -> Kind {
        Kind::LBrack
    }

    fn close(&self) -> Kind {
        Kind::RBrack
    }

    fn parse_element(&mut self, p: &mut Parser<'a>) -> Result<()> {
        let element = p
            .parse_assignment_expression_base()
            .map(ArrayExpressionElement::Expression);

        if p.at(Kind::Comma) && p.peek_at(self.close()) {
            self.trailing_comma = Some(p.end_span(p.start_span()));
        }

        self.elements.push(element?);
        Ok(())
    }
}

pub struct CallArguments<'a> {
    pub elements: Vec<'a, Argument<'a>>,
    pub rest_element_with_trilling_comma: Option<Span>,
}

impl<'a> SeparatedList<'a> for CallArguments<'a> {
    fn new(p: &Parser<'a>) -> Self {
        Self {
            elements: p.ast.new_vec(),
            rest_element_with_trilling_comma: None,
        }
    }

    fn open(&self) -> Kind {
        Kind::LParen
    }

    fn close(&self) -> Kind {
        Kind::RParen
    }

    fn parse_element(&mut self, p: &mut Parser<'a>) -> Result<()> {
        let element = p
            .parse_assignment_expression_base()
            .map(Argument::Expression);
        self.elements.push(element?);
        Ok(())
    }
}

pub struct ClassElements<'a> {
    pub elements: Vec<'a, ClassElement<'a>>,
}

impl<'a> ClassElements<'a> {
    pub(crate) fn new(p: &Parser<'a>) -> Self {
        Self {
            elements: p.ast.new_vec(),
        }
    }
}

impl<'a> NormalList<'a> for ClassElements<'a> {
    fn open(&self) -> Kind {
        Kind::LCurly
    }

    fn close(&self) -> Kind {
        Kind::RCurly
    }

    fn parse_element(&mut self, p: &mut Parser<'a>) -> Result<()> {
        // skip empty class element `;`
        while p.at(Kind::Semicolon) {
            p.bump_any();
        }

        if p.at(self.close()) {
            return Ok(());
        }

        let element = p.parse_class_element()?;

        self.elements.push(element);
        Ok(())
    }
}

pub struct ImportSpecifierList<'a> {
    pub import_specifiers: Vec<'a, ImportDeclarationSpecifier>,
}

impl<'a> SeparatedList<'a> for ImportSpecifierList<'a> {
    fn new(p: &Parser<'a>) -> Self {
        Self {
            import_specifiers: p.ast.new_vec(),
        }
    }

    fn open(&self) -> Kind {
        Kind::LCurly
    }

    fn close(&self) -> Kind {
        Kind::RCurly
    }

    fn parse_element(&mut self, p: &mut Parser<'a>) -> Result<()> {
        let import_specifier = p.parse_import_specifier()?;
        let specifier = ImportDeclarationSpecifier::ImportSpecifier(import_specifier);
        self.import_specifiers.push(specifier);
        Ok(())
    }
}
