mod binding;
mod context;
mod cursor;
mod declaration;
mod diagnostics;
mod expression;
mod function;
mod grammar;
mod list;
mod operator;
mod statement;

use context::StatementContext;
use wave_allocator::Allocator;
use wave_ast::{
    ast::{Expression, Program, Statement},
    ast_builder::AstBuilder,
    Trivias,
};
use wave_diagnostics::{Error, Result};
use wave_lexer::{Kind, Lexer, Token};
use wave_span::Span;

pub struct ParserReturn<'a> {
    pub program: Program<'a>,
    pub errors: Vec<Error>,
    pub trivias: Trivias,
    pub panicked: bool,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    source_text: &'a str,
    errors: Vec<Error>,
    token: Token,
    prev_token_end: u32,
    ast: AstBuilder<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(allocator: &'a Allocator, source_text: &'a str) -> Self {
        Self {
            lexer: Lexer::new(allocator, source_text),
            source_text,
            errors: vec![],
            token: Token::default(),
            prev_token_end: 0,
            ast: AstBuilder::new(allocator),
        }
    }

    fn error<T: Into<Error>>(&mut self, error: T) {
        self.errors.push(error.into());
    }

    pub fn parse(mut self) -> ParserReturn<'a> {
        let (program, panicked) = match self.parse_program() {
            Ok(program) => (program, false),
            Err(error) => {
                self.error(error);
                let program = self.ast.program(Span::default(), self.ast.new_vec());
                (program, true)
            }
        };

        let errors = self.lexer.errors.into_iter().chain(self.errors).collect();
        let trivias = self.lexer.trivia_builder.build();

        ParserReturn {
            program,
            errors,
            trivias,
            panicked,
        }
    }

    fn parse_program(&mut self) -> Result<Program<'a>> {
        self.bump_any();

        let statements = self.parse_statements()?;

        let span = Span::new(0, self.source_text.len() as u32);
        Ok(self.ast.program(span, statements))
    }

    fn parse_statements(&mut self) -> Result<wave_allocator::Vec<'a, Statement<'a>>> {
        let mut statements = self.ast.new_vec();

        while !self.at(Kind::Eof) {
            match self.cur_kind() {
                Kind::RCurly => break,
                _ => {
                    let stmt = self.parse_statement_list_item(StatementContext::StatementList)?;

                    if let Statement::ExpressionStatement(expr) = &stmt {
                        if let Expression::StringLiteral(string) = &expr.expression {
                            if expr.span.start == string.span.start {
                                let _ = &self.source_text
                                    [string.span.start as usize + 1..string.span.end as usize - 1];
                                continue;
                            }
                        }
                    }
                    statements.push(stmt);
                }
            }
        }

        Ok(statements)
    }

    fn unexpected(&mut self) -> Error {
        if self.cur_kind() == Kind::Undetermined {
            if let Some(error) = self.lexer.errors.pop() {
                return error;
            }
        }
        diagnostics::UnexpectedToken(self.cur_token().span()).into()
    }
}
