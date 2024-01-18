use wave_allocator::{Allocator, Box, Vec};
use wave_ast::{
    ast::{
        BindingPatternKind, Declaration, Expression, ExpressionStatement, IdentifierReference,
        Program, Statement, VariableDeclaration, VariableDeclarator,
    },
    BooleanLiteral,
};
use wave_diagnostics::Result;

use crate::{environment::Environment, Runtime};

#[derive(Debug, PartialEq, Clone)]
pub enum ER {
    Number(f64),
    Boolean(bool),
    String(String),
    Null,
}

pub fn load_environment(environment: &mut Box<'_, Environment>) {
    environment.define("PI".into(), ER::Number(3.4));
}

pub fn eval_runtime(runtime: Runtime) -> Result<ER> {
    let mut environment = Box(runtime.arena.alloc(Environment::default()));
    load_environment(&mut environment);
    eval_program(&runtime.program, &mut environment, runtime.arena)
}

pub fn eval_program(
    program: &Program<'_>,
    environment: &mut Box<'_, Environment>,
    arena: &Allocator,
) -> Result<ER> {
    let mut result = ER::Null;
    for statement in &program.body {
        result = eval_statement(statement, environment, arena)?;
    }
    Ok(result)
}

fn eval_statement(
    statement: &Statement<'_>,
    environment: &mut Box<'_, Environment>,
    arena: &Allocator,
) -> Result<ER> {
    match statement {
        Statement::ExpressionStatement(expression_stmt) => {
            let result = eval_expression_statement(expression_stmt, environment, arena)?;
            Ok(result)
        }
        Statement::Declaration(declaration) => {
            let result = eval_declaration(declaration, environment, arena)?;
            Ok(result)
        }
        _ => unimplemented!(),
    }
}

fn eval_declaration(
    declaration: &Declaration<'_>,
    environment: &mut Box<'_, Environment>,
    arena: &Allocator,
) -> Result<ER> {
    match declaration {
        Declaration::VariableDeclaration(declaration) => {
            let result = eval_variable_declaration(declaration, environment, arena)?;
            Ok(result)
        }
        _ => unimplemented!("declaration"),
    }
}

fn eval_variable_declaration(
    declaration: &Box<'_, VariableDeclaration>,
    environment: &mut Box<'_, Environment>,
    arena: &Allocator,
) -> Result<ER> {
    eval_variable_declarator(&declaration.declarations, environment, arena)?;
    Ok(ER::Null)
}

fn eval_variable_declarator(
    declarators: &Vec<'_, VariableDeclarator>,
    environment: &mut Box<'_, Environment>,
    arena: &Allocator,
) -> Result<ER> {
    for declarator in declarators {
        if declarator.init.is_none() {
            continue;
        }
        match &declarator.id.kind {
            BindingPatternKind::BindingIdentifier(identifier) => {
                let value = eval_expression(declarator.init.as_ref().unwrap(), environment, arena)?;
                environment.define(identifier.name.to_owned(), value);
            }
        }
    }
    Ok(ER::Null)
}

fn eval_expression_statement(
    expression_stmt: &Box<'_, ExpressionStatement>,
    environment: &mut Box<'_, Environment>,
    arena: &Allocator,
) -> Result<ER> {
    eval_expression(&expression_stmt.expression, environment, arena)
}

fn eval_expression(
    expression: &Expression<'_>,
    environment: &mut Box<'_, Environment>,
    _arena: &Allocator,
) -> Result<ER> {
    match expression {
        Expression::BooleanLiteral(expression) => eval_boolean_literal(expression),
        Expression::Identifier(expression) => eval_identifier(expression, environment),
        _ => unimplemented!(),
    }
}

fn eval_boolean_literal(expression: &Box<'_, BooleanLiteral>) -> Result<ER> {
    Ok(ER::Boolean(expression.value))
}

fn eval_identifier(
    expression: &Box<'_, IdentifierReference>,
    environment: &Box<'_, Environment>,
) -> Result<ER> {
    let value = environment.get(expression.name.to_owned());
    match value {
        Some(value) => Ok(value.clone()),
        None => Ok(ER::Null),
    }
}
