use wave_allocator::Box;
use wave_ast::ast::{BlockStatement, Declaration, Expression, Statement};
use wave_diagnostics::Result;
use wave_lexer::Kind;
use wave_span::Span;

use crate::{
    context::StatementContext,
    declaration::{VariableDeclarationContext, VariableDeclarationParent},
    diagnostics, Parser,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_statement_list_item(
        &mut self,
        stmt_ctx: StatementContext,
    ) -> Result<Statement<'a>> {
        match self.cur_kind() {
            Kind::LCurly => self.parse_block_statement(),
            Kind::If => self.parse_if_statement(),
            Kind::Const => self.parse_variable_statement(stmt_ctx),
            Kind::Let => self.parse_variable_statement(stmt_ctx),
            _ => self.parse_expression_or_labeled_statement(),
        }
    }

    fn parse_expression_or_labeled_statement(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();
        let expr = self.parse_expression()?;
        self.parse_expression_statement(span, expr)
    }

    pub(crate) fn parse_expression_statement(
        &mut self,
        span: Span,
        expression: Expression<'a>,
    ) -> Result<Statement<'a>> {
        self.asi()?;
        Ok(self
            .ast
            .expression_statement(self.end_span(span), expression))
    }

    pub(crate) fn parse_variable_statement(
        &mut self,
        stmt_ctx: StatementContext,
    ) -> Result<Statement<'a>> {
        let start_span = self.start_span();

        let decl = self.parse_variable_declaration(
            start_span,
            VariableDeclarationContext::new(VariableDeclarationParent::Statement),
        )?;

        if stmt_ctx.is_single_statement() && decl.kind.is_lexical() {
            self.error(diagnostics::LexicalDeclarationSingleStatement(decl.span));
        }

        Ok(Statement::Declaration(Declaration::VariableDeclaration(
            decl,
        )))
    }

    fn parse_empty_statement(&mut self) -> Statement<'a> {
        let span = self.start_span();
        self.bump_any(); // bump `;`
        self.ast.empty_statement(self.end_span(span))
    }

    fn parse_if_statement(&mut self) -> Result<Statement<'a>> {
        let span = self.start_span();
        self.bump_any(); // bump `if`
        let condition = self.parse_paren_expression()?;
        let consequent = self.parse_statement_list_item(StatementContext::If)?;
        let alternate = self
            .eat(Kind::Else)
            .then(|| self.parse_statement_list_item(StatementContext::If))
            .transpose()?;
        Ok(self
            .ast
            .if_statement(self.end_span(span), condition, consequent, alternate))
    }

    pub(crate) fn parse_block_statement(&mut self) -> Result<Statement<'a>> {
        let block = self.parse_block()?;
        Ok(self.ast.block_statement(block))
    }

    pub(crate) fn parse_block(&mut self) -> Result<Box<'a, BlockStatement<'a>>> {
        let span = self.start_span();
        self.expect(Kind::LCurly)?;
        let mut body = self.ast.new_vec();
        while !self.at(Kind::RCurly) && !self.at(Kind::Eof) {
            let stmt = self.parse_statement_list_item(StatementContext::StatementList)?;
            body.push(stmt);
        }
        self.expect(Kind::RCurly)?;
        Ok(self.ast.block(self.end_span(span), body))
    }
}
