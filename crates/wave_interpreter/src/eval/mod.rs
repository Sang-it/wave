use wave_allocator::{Allocator, Box, Vec};
use wave_ast::{
    ast::{
        ArrayExpression, ArrayExpressionElement, BindingPatternKind, Declaration, Expression,
        ExpressionStatement, IdentifierReference, Program, Statement, VariableDeclaration,
        VariableDeclarator,
    },
    BooleanLiteral, NumberLiteral, StringLiteral,
};
use wave_diagnostics::Result;

use crate::{environment::Environment, Runtime};
use std::vec::Vec as SVec;

#[derive(Debug, PartialEq, Clone)]
pub enum ER {
    Number(f64),
    Boolean(bool),
    String(String),
    Array(SVec<ER>),
    Null,
}

pub fn load_environment(environment: &mut Box<'_, Environment>) {
    environment.define("PI".into(), ER::Number(3.4));
}

pub fn eval_runtime(runtime: Runtime) -> Result<ER> {
    let mut environment = Box(runtime.arena.alloc(Environment::default()));
    load_environment(&mut environment);
    eval_program(&runtime.program, &mut environment)
}

pub fn eval_program(program: &Program<'_>, environment: &mut Box<'_, Environment>) -> Result<ER> {
    let mut result = ER::Null;
    for statement in &program.body {
        result = eval_statement(statement, environment)?;
    }
    Ok(result)
}

fn eval_statement(statement: &Statement<'_>, environment: &mut Box<'_, Environment>) -> Result<ER> {
    match statement {
        Statement::ExpressionStatement(expression_stmt) => {
            let result = eval_expression_statement(expression_stmt, environment)?;
            Ok(result)
        }
        Statement::Declaration(declaration) => {
            let result = eval_declaration(declaration, environment)?;
            Ok(result)
        }
        _ => unimplemented!(),
    }
}

fn eval_declaration(
    declaration: &Declaration<'_>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER> {
    match declaration {
        Declaration::VariableDeclaration(declaration) => {
            let result = eval_variable_declaration(declaration, environment)?;
            Ok(result)
        }
        _ => unimplemented!("declaration"),
    }
}

fn eval_variable_declaration(
    declaration: &Box<'_, VariableDeclaration>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER> {
    eval_variable_declarator(&declaration.declarations, environment)?;
    Ok(ER::Null)
}

fn eval_variable_declarator(
    declarators: &Vec<'_, VariableDeclarator>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER> {
    for declarator in declarators {
        if declarator.init.is_none() {
            continue;
        }
        match &declarator.id.kind {
            BindingPatternKind::BindingIdentifier(identifier) => {
                let value = eval_expression(declarator.init.as_ref().unwrap(), environment)?;
                environment.define(identifier.name.to_owned(), value);
            }
        }
    }
    Ok(ER::Null)
}

fn eval_expression_statement(
    expression_stmt: &Box<'_, ExpressionStatement>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER> {
    eval_expression(&expression_stmt.expression, environment)
}

fn eval_expression(
    expression: &Expression<'_>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER> {
    match expression {
        Expression::BooleanLiteral(expression) => eval_boolean_literal(expression),
        Expression::NumberLiteral(expression) => eval_number_literal(expression),
        Expression::StringLiteral(expression) => eval_string_literal(expression),
        Expression::Identifier(expression) => eval_identifier(expression, environment),
        Expression::ArrayExpression(expression) => eval_array_expression(expression, environment),
        _ => unimplemented!(),
    }
}

fn eval_boolean_literal(expression: &Box<'_, BooleanLiteral>) -> Result<ER> {
    Ok(ER::Boolean(expression.value))
}

fn eval_number_literal(expression: &Box<'_, NumberLiteral>) -> Result<ER> {
    Ok(ER::Number(expression.value))
}

fn eval_string_literal(expression: &Box<'_, StringLiteral>) -> Result<ER> {
    Ok(ER::String(expression.value.to_string()))
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

fn eval_array_expression(
    expression: &Box<'_, ArrayExpression>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER> {
    let mut result = SVec::new();
    for element in &expression.elements {
        let value = eval_array_expression_element(element, environment)?;
        result.push(value);
    }
    Ok(ER::Array(result))
}

fn eval_array_expression_element(
    expression: &ArrayExpressionElement<'_>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER> {
    match expression {
        ArrayExpressionElement::Expression(expression) => eval_expression(expression, environment),
    }
}
