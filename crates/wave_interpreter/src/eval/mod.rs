use wave_allocator::{Allocator, Box, Vec};
use wave_ast::{
    ast::{Expression, ExpressionStatement, IdentifierReference, Program, Statement},
    BooleanLiteral,
};
use wave_diagnostics::Result;

use crate::{environment::Environment, Runtime};

#[derive(Debug, PartialEq, Clone)]
pub enum ER<'a> {
    Number(f64),
    Boolean(bool),
    String(String),
    Array(Vec<'a, ER<'a>>),
    Null,
}

pub fn load_environment(environment: &mut Box<'_, Environment<'_>>) {
    environment.define("PI".into(), ER::Number(3.4));
}

pub fn eval_runtime(runtime: Runtime) -> Result<ER> {
    let mut environment = Box(runtime.arena.alloc(Environment::default()));
    load_environment(&mut environment);
    eval_program(&runtime.program, &mut environment, runtime.arena)
}

pub fn eval_program<'a>(
    program: &Program<'_>,
    environment: &mut Box<'a, Environment<'a>>,
    arena: &Allocator,
) -> Result<ER<'a>> {
    let mut result = ER::Null;
    for statement in &program.body {
        result = eval_statement(statement, environment, arena)?;
    }
    Ok(result)
}

fn eval_statement<'a>(
    statement: &Statement<'_>,
    environment: &mut Box<'a, Environment<'a>>,
    arena: &Allocator,
) -> Result<ER<'a>> {
    match statement {
        Statement::ExpressionStatement(expression_stmt) => {
            let result = eval_expression_statement(expression_stmt, environment, arena)?;
            Ok(result)
        }
        _ => unimplemented!(),
    }
}

fn eval_expression_statement<'a>(
    expression_stmt: &Box<'_, ExpressionStatement>,
    environment: &mut Box<'a, Environment<'a>>,
    arena: &Allocator,
) -> Result<ER<'a>> {
    eval_expression(&expression_stmt.expression, environment, arena)
}

fn eval_expression<'a>(
    expression: &Expression<'_>,
    environment: &mut Box<'a, Environment<'a>>,
    _arena: &Allocator,
) -> Result<ER<'a>> {
    match expression {
        Expression::BooleanLiteral(expression) => eval_boolean_literal(expression),
        Expression::Identifier(expression) => eval_identifier(expression, environment),
        _ => unimplemented!(),
    }
}

fn eval_boolean_literal<'a>(expression: &Box<'_, BooleanLiteral>) -> Result<ER<'a>> {
    Ok(ER::Boolean(expression.value))
}

fn eval_identifier<'a>(
    expression: &Box<'_, IdentifierReference>,
    environment: &Box<'a, Environment<'a>>,
) -> Result<ER<'a>> {
    let value = environment.get(expression.name.to_owned());
    match value {
        Some(value) => Ok(value.clone()),
        None => Ok(ER::Null),
    }
}
