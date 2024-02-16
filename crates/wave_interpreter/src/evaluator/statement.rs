use std::cell::RefCell;
use std::rc::Rc;

use crate::evaluator::Primitive;
use crate::Runtime;
use crate::{diagnostics, environment::Environment};
use std::boxed::Box as StdBox;
use wave_allocator::{Box, Vec};
use wave_ast::ast::{ExpressionStatement, IfStatement, ReturnStatement, Statement, WhileStatement};
use wave_diagnostics::Result;
use wave_span::GetSpan;

impl<'a> Runtime<'a> {
    pub fn eval_statement(
        &self,
        statement: &Statement<'a>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match statement {
            Statement::ExpressionStatement(expression_stmt) => {
                self.eval_expression_statement(expression_stmt, environment)
            }
            Statement::Declaration(declaration) => self.eval_declaration(declaration, environment),
            Statement::IfStatement(if_stmt) => self.eval_if_statement(if_stmt, environment),
            Statement::BlockStatement(block_stmt) => self.eval_block(&block_stmt.body, environment),
            Statement::ReturnStatement(return_stmt) => {
                self.eval_return_statement(return_stmt, environment)
            }
            Statement::WhileStatement(while_stmt) => {
                self.eval_while_statement(while_stmt, environment)
            }
            Statement::BreakStatement(_) => Ok(Primitive::Break),
            Statement::ContinueStatement(_) => Ok(Primitive::Continue),
            Statement::ModuleDeclaration(import_stmt) => {
                self.eval_import_statement(import_stmt, environment)
            }
        }
    }

    fn eval_expression_statement(
        &self,
        expression_stmt: &Box<'_, ExpressionStatement<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        self.eval_expression(&expression_stmt.expression, environment)
    }

    pub fn eval_block(
        &self,
        body: &Vec<'_, Statement<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        for statement in body {
            let result = self.eval_statement(statement, Rc::clone(&environment))?;
            if matches!(result, Primitive::Return(_)) {
                return Ok(result);
            }
            if matches!(result, Primitive::Break | Primitive::Continue) {
                return Ok(result);
            }
        }
        Ok(Primitive::Null)
    }

    pub fn eval_if_statement(
        &self,
        if_stmt: &Box<'_, IfStatement<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let test = self.eval_expression(&if_stmt.test, Rc::clone(&environment))?;
        match test {
            Primitive::Boolean(true) => self.eval_statement(&if_stmt.consequent, environment),
            Primitive::Boolean(false) => {
                if let Some(alternate) = &if_stmt.alternate {
                    self.eval_statement(alternate, environment)
                } else {
                    Ok(Primitive::Null)
                }
            }
            _ => Err(diagnostics::InvalidBoolean(if_stmt.test.span()).into()),
        }
    }

    pub fn eval_return_statement(
        &self,
        return_stmt: &Box<'_, ReturnStatement<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        if let Some(expression) = &return_stmt.argument {
            let eval = self.eval_expression(expression, environment)?;
            Ok(Primitive::Return(StdBox::new(eval)))
        } else {
            Ok(Primitive::Return(StdBox::new(Primitive::Null)))
        }
    }

    pub fn eval_while_statement(
        &self,
        while_stmt: &Box<'_, WhileStatement<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let test = self.eval_expression(&while_stmt.test, Rc::clone(&environment))?;
        match test {
            Primitive::Boolean(true) => {
                let eval = self.eval_statement(&while_stmt.body, Rc::clone(&environment))?;
                if !matches!(eval, Primitive::Break) {
                    let _ = self.eval_while_statement(while_stmt, environment);
                }
                Ok(Primitive::Null)
            }
            Primitive::Boolean(false) => Ok(Primitive::Null),
            _ => Err(diagnostics::InvalidBoolean(while_stmt.test.span()).into()),
        }
    }
}
