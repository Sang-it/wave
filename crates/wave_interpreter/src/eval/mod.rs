use crate::diagnostics;
use crate::{environment::Environment, Runtime};
use std::ptr;
use std::{f64::consts::PI, vec::Vec as SVec};
use wave_allocator::{Box, Vec};
use wave_ast::ast::{CallExpression, Function, IfStatement};
use wave_ast::{
    ast::{
        ArrayExpression, ArrayExpressionElement, BinaryExpression, BindingPatternKind, Declaration,
        Expression, ExpressionStatement, IdentifierReference, LogicalExpression, Program,
        Statement, VariableDeclaration, VariableDeclarator,
    },
    BooleanLiteral, NumberLiteral, StringLiteral,
};
use wave_diagnostics::Result;
use wave_span::GetSpan;
use wave_syntax::operator::{BinaryOperator, LogicalOperator};

#[derive(Debug)]
pub enum ER<'a> {
    Number(f64),
    Boolean(bool),
    String(String),
    Array(SVec<ER<'a>>),
    Function(Function<'a>),
    Null,
}

impl<'a> Clone for ER<'a> {
    fn clone(&self) -> Self {
        match self {
            ER::Number(value) => ER::Number(*value),
            ER::Boolean(value) => ER::Boolean(*value),
            ER::String(value) => ER::String(value.to_owned()),
            ER::Array(value) => ER::Array(value.to_owned()),
            ER::Function(value) =>
            // TODO: This should be safe -- I think.
            unsafe {
                let function = ptr::read(value);
                ER::Function(function)
            },
            ER::Null => ER::Null,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

pub fn load_environment(environment: &mut Box<'_, Environment>) {
    environment.define("PI".into(), ER::Number(PI));
}

pub fn eval_runtime(runtime: Runtime) -> Result<ER> {
    let mut environment = Box(runtime.arena.alloc(Environment::default()));
    load_environment(&mut environment);
    eval_program(&runtime.program, &mut environment)
}

pub fn eval_program<'a>(
    program: &Program<'a>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    let mut result = ER::Null;
    for statement in &program.body {
        result = eval_statement(statement, environment)?;
    }
    Ok(result)
}

fn eval_statement<'a>(
    statement: &Statement<'a>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    match statement {
        Statement::ExpressionStatement(expression_stmt) => {
            let result = eval_expression_statement(expression_stmt, environment)?;
            Ok(result)
        }
        Statement::Declaration(declaration) => {
            let result = eval_declaration(declaration, environment)?;
            Ok(result)
        }
        Statement::IfStatement(if_stmt) => {
            let result = eval_if_statement(if_stmt, environment)?;
            Ok(result)
        }
        Statement::BlockStatement(block_stmt) => {
            let mut result = ER::Null;
            for statement in &block_stmt.body {
                result = eval_statement(statement, environment)?;
            }
            Ok(result)
        }
        _ => unimplemented!("eval_statement"),
    }
}

fn eval_declaration<'a>(
    declaration: &Declaration<'a>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    match declaration {
        Declaration::VariableDeclaration(declaration) => {
            let result = eval_variable_declaration(declaration, environment)?;
            Ok(result)
        }
        Declaration::FunctionDeclaration(declaration) => {
            eval_function_declaration(declaration, environment)
        }
        _ => unimplemented!("declaration"),
    }
}

fn eval_variable_declaration<'a>(
    declaration: &Box<'_, VariableDeclaration>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    eval_variable_declarator(&declaration.declarations, environment)?;
    Ok(ER::Null)
}

fn eval_function_declaration<'a>(
    declaration: &Box<'_, Function<'a>>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    unsafe {
        let function = ptr::read(declaration).unbox();
        if let Some(id) = &function.id {
            environment.define(id.name.to_owned(), ER::Function(function));
        }
    }
    Ok(ER::Null)
}

fn eval_variable_declarator<'a>(
    declarators: &Vec<'_, VariableDeclarator>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER<'a>> {
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

fn eval_expression_statement<'a>(
    expression_stmt: &Box<'_, ExpressionStatement>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    eval_expression(&expression_stmt.expression, environment)
}

fn eval_expression<'a>(
    expression: &Expression<'_>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    match expression {
        Expression::BooleanLiteral(expression) => eval_boolean_literal(expression),
        Expression::NumberLiteral(expression) => eval_number_literal(expression),
        Expression::StringLiteral(expression) => eval_string_literal(expression),
        Expression::Identifier(expression) => eval_identifier(expression, environment),
        Expression::ArrayExpression(expression) => eval_array_expression(expression, environment),
        Expression::BinaryExpression(expression) => eval_binary_expression(expression, environment),
        Expression::LogicalExpression(expression) => {
            eval_logical_expression(expression, environment)
        }
        Expression::CallExpression(expression) => eval_call_expression(expression, environment),
        _ => unimplemented!(),
    }
}

fn eval_boolean_literal<'a>(expression: &Box<'_, BooleanLiteral>) -> Result<ER<'a>> {
    Ok(ER::Boolean(expression.value))
}

fn eval_number_literal<'a>(expression: &Box<'_, NumberLiteral>) -> Result<ER<'a>> {
    Ok(ER::Number(expression.value))
}

fn eval_string_literal<'a>(expression: &Box<'_, StringLiteral>) -> Result<ER<'a>> {
    Ok(ER::String(expression.value.to_string()))
}

fn eval_identifier<'a>(
    expression: &Box<'_, IdentifierReference>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    environment
        .get(expression.name.to_owned(), expression.span)
        .map(|v| v.clone())
}

fn eval_array_expression<'a>(
    expression: &Box<'_, ArrayExpression>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    let mut result = SVec::new();
    for element in &expression.elements {
        let value = eval_array_expression_element(element, environment)?;
        result.push(value);
    }
    Ok(ER::Array(result))
}

fn eval_array_expression_element<'a>(
    expression: &ArrayExpressionElement<'_>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    match expression {
        ArrayExpressionElement::Expression(expression) => eval_expression(expression, environment),
    }
}

fn eval_binary_expression<'a>(
    expression: &BinaryExpression<'_>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER<'a>> {
    let left = &expression.left;
    let right = &expression.right;
    match expression.operator {
        BinaryOperator::Addition
        | BinaryOperator::Subtraction
        | BinaryOperator::Multiplication
        | BinaryOperator::Division
        | BinaryOperator::Remainder
        | BinaryOperator::Exponential => {
            eval_arithmetic(left, right, environment, &expression.operator)
        }

        BinaryOperator::Equality
        | BinaryOperator::Inequality
        | BinaryOperator::LessThan
        | BinaryOperator::LessEqualThan
        | BinaryOperator::GreaterThan
        | BinaryOperator::GreaterEqualThan => {
            eval_ord(left, right, environment, &expression.operator)
        }

        BinaryOperator::BitwiseOR | BinaryOperator::BitwiseAnd | BinaryOperator::BitwiseXOR => {
            eval_bitwise(left, right, environment, &expression.operator)
        }
    }
}

// Arithmetic operations
fn eval_arithmetic<'a>(
    left: &Expression<'_>,
    right: &Expression<'_>,
    environment: &mut Box<'_, Environment>,
    operator: &BinaryOperator,
) -> Result<ER<'a>> {
    let l = eval_expression(left, environment)?;
    let r = eval_expression(right, environment)?;

    match (l, r) {
        (ER::Number(left), ER::Number(right)) => match operator {
            BinaryOperator::Addition => Ok(ER::Number(left + right)),
            BinaryOperator::Subtraction => Ok(ER::Number(left - right)),
            BinaryOperator::Multiplication => Ok(ER::Number(left * right)),
            BinaryOperator::Division => Ok(ER::Number(left / right)),
            BinaryOperator::Remainder => Ok(ER::Number(left % right)),
            BinaryOperator::Exponential => Ok(ER::Number(left.powf(right))),
            _ => unreachable!(),
        },
        _ => Err(diagnostics::InvalidNumber(left.span().merge(&right.span())).into()),
    }
}

// Ord operations
fn eval_ord<'a>(
    left: &Expression<'_>,
    right: &Expression<'_>,
    environment: &mut Box<'_, Environment>,
    operator: &BinaryOperator,
) -> Result<ER<'a>> {
    let l = eval_expression(left, environment)?;
    let r = eval_expression(right, environment)?;

    match (l, r) {
        (ER::Number(l), ER::Number(r)) => match operator {
            BinaryOperator::LessThan => Ok(ER::Boolean(l < r)),
            BinaryOperator::LessEqualThan => Ok(ER::Boolean(l <= r)),
            BinaryOperator::GreaterThan => Ok(ER::Boolean(l > r)),
            BinaryOperator::GreaterEqualThan => Ok(ER::Boolean(l >= r)),
            BinaryOperator::Equality => Ok(ER::Boolean(l == r)),
            BinaryOperator::Inequality => Ok(ER::Boolean(l != r)),
            _ => unreachable!(),
        },
        (ER::Boolean(l), ER::Boolean(r)) => match operator {
            BinaryOperator::Equality => Ok(ER::Boolean(l == r)),
            BinaryOperator::Inequality => Ok(ER::Boolean(l != r)),
            _ => Err(diagnostics::InvalidNumber(left.span().merge(&right.span())).into()),
        },

        (ER::String(l), ER::String(r)) => match operator {
            BinaryOperator::Equality => Ok(ER::Boolean(l == r)),
            BinaryOperator::Inequality => Ok(ER::Boolean(l != r)),
            _ => Err(diagnostics::InvalidNumber(left.span().merge(&right.span())).into()),
        },
        _ => Err(diagnostics::TypeMismatch(left.span().merge(&right.span())).into()),
    }
}

// Bitwise operations
fn eval_bitwise<'a>(
    left: &Expression<'_>,
    right: &Expression<'_>,
    environment: &mut Box<'_, Environment>,
    operator: &BinaryOperator,
) -> Result<ER<'a>> {
    let l = eval_expression(left, environment)?;
    let r = eval_expression(right, environment)?;

    match (l, r) {
        (ER::Number(left), ER::Number(right)) => match operator {
            BinaryOperator::BitwiseOR => Ok(ER::Number((left as u64 | right as u64) as f64)),
            BinaryOperator::BitwiseAnd => Ok(ER::Number((left as u64 & right as u64) as f64)),
            BinaryOperator::BitwiseXOR => Ok(ER::Number((left as u64 ^ right as u64) as f64)),
            _ => unreachable!(),
        },
        _ => Err(diagnostics::InvalidNumber(left.span().merge(&right.span())).into()),
    }
}

fn eval_logical_expression<'a>(
    expression: &LogicalExpression<'_>,
    environment: &mut Box<'_, Environment>,
) -> Result<ER<'a>> {
    let left = &expression.left;
    let right = &expression.right;

    match expression.operator {
        LogicalOperator::Or | LogicalOperator::And => {
            eval_logical(left, right, environment, &expression.operator)
        }
    }
}

fn eval_logical<'a>(
    left: &Expression<'_>,
    right: &Expression<'_>,
    environment: &mut Box<'_, Environment>,
    operator: &LogicalOperator,
) -> Result<ER<'a>> {
    let l = eval_expression(left, environment)?;
    let r = eval_expression(right, environment)?;

    match (l, r) {
        (ER::Boolean(left), ER::Boolean(right)) => match operator {
            LogicalOperator::Or => Ok(ER::Boolean(left || right)),
            LogicalOperator::And => Ok(ER::Boolean(left && right)),
        },
        _ => Err(diagnostics::InvalidBoolean(left.span().merge(&right.span())).into()),
    }
}

fn eval_if_statement<'a>(
    if_stmt: &Box<'_, IfStatement<'a>>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    let test = eval_expression(&if_stmt.test, environment)?;
    match test {
        ER::Boolean(true) => eval_statement(&if_stmt.consequent, environment),
        ER::Boolean(false) => {
            if let Some(alternate) = &if_stmt.alternate {
                eval_statement(alternate, environment)
            } else {
                Ok(ER::Null)
            }
        }
        _ => Err(diagnostics::InvalidBoolean(if_stmt.test.span()).into()),
    }
}

fn eval_call_expression<'a>(
    expression: &Box<'_, CallExpression>,
    environment: &mut Box<'_, Environment<'a>>,
) -> Result<ER<'a>> {
    let calle = &expression.callee;

    match calle {
        Expression::Identifier(identifier) => unsafe {
            let mut result = ER::Null;

            // TODO: This should be safe -- I think.
            let env = ptr::read(environment);
            let function = env.get(identifier.name.to_owned(), identifier.span)?;

            // environment.extend(env);

            let body = match function {
                ER::Function(function) => &function.body,
                _ => unreachable!(),
            };

            if let Some(body) = body {
                for statement in &body.statements {
                    result = eval_statement(statement, environment)?;
                }
            }
            Ok(result)
        },
        _ => unimplemented!(),
    }
}
