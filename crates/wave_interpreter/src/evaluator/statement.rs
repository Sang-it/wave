use std::cell::RefCell;
use std::rc::Rc;

use crate::evaluator::Primitive;
use crate::Runtime;
use crate::{diagnostics, environment::Environment};
use wave_allocator::{Box, Vec};
use wave_ast::ast::{ExpressionStatement, IfStatement, ReturnStatement, Statement};
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
            Statement::ReturnStatement(return_stmt) => self.eval_return(return_stmt, environment),
            _ => unimplemented!("eval_statement"),
        }
    }

    fn eval_expression_statement(
        &self,
        expression_stmt: &Box<'_, ExpressionStatement>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        self.eval_expression(&expression_stmt.expression, environment)
    }

    pub fn eval_block(
        &self,
        body: &Vec<'_, Statement<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let mut result = Primitive::Null;
        for statement in body {
            result = self.eval_statement(statement, Rc::clone(&environment))?;
            if matches!(statement, Statement::ReturnStatement(_)) {
                return Ok(result);
            }
        }
        Ok(result)
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

    pub fn eval_return(
        &self,
        return_stmt: &Box<'_, ReturnStatement<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        if let Some(expression) = &return_stmt.argument {
            self.eval_expression(expression, environment)
        } else {
            Ok(Primitive::Null)
        }
    }
}
