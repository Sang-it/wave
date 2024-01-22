pub mod eval_result;

use crate::{diagnostics, environment::Environment, Runtime};
use eval_result::ER;
use std::{cell::RefCell, f64::consts::PI, ptr, rc::Rc, vec::Vec as StdVec};
use wave_allocator::{Box, Vec};
use wave_ast::{
    ast::{
        Argument, ArrayExpression, ArrayExpressionElement, BinaryExpression, BindingPatternKind,
        CallExpression, Declaration, Expression, ExpressionStatement, FormalParameter, Function,
        IdentifierReference, IfStatement, LogicalExpression, Program, Statement,
        VariableDeclaration, VariableDeclarator,
    },
    BooleanLiteral, NumberLiteral, StringLiteral,
};
use wave_diagnostics::Result;
use wave_span::{Atom, GetSpan, Span};
use wave_syntax::operator::{BinaryOperator, LogicalOperator};

pub fn load_environment(environment: &mut Box<'_, Environment>) {
    environment.define("PI".into(), ER::Number(PI));
}

pub fn eval_runtime(runtime: Runtime) -> Result<ER> {
    let environment = Rc::new(RefCell::new(Environment::default()));
    eval_program(&runtime.program, environment)
}

pub fn eval_program<'a>(
    program: &Program<'a>,
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    let mut result = ER::Null;
    for statement in &program.body {
        result = eval_statement(statement, Rc::clone(&environment))?;
    }
    Ok(result)
}

#[rustfmt::skip]
fn eval_statement<'a>(
    statement: &Statement<'a>,
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    match statement {
        Statement::ExpressionStatement(expression_stmt) => eval_expression_statement(expression_stmt, environment),
        Statement::Declaration(declaration) => eval_declaration(declaration, environment),
        Statement::IfStatement(if_stmt) => eval_if_statement(if_stmt, environment),
        Statement::BlockStatement(block_stmt) => eval_block_body(&block_stmt.body, environment),
        _ => unimplemented!("eval_statement"),
    }
}

fn eval_block_body<'a>(
    body: &Vec<'_, Statement<'a>>,
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    let mut result = ER::Null;
    for statement in body {
        result = eval_statement(statement, Rc::clone(&environment))?;
    }
    Ok(result)
}

fn eval_declaration<'a>(
    declaration: &Declaration<'a>,
    environment: Rc<RefCell<Environment<'a>>>,
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
    environment: Rc<RefCell<Environment>>,
) -> Result<ER<'a>> {
    eval_variable_declarator(&declaration.declarations, environment)?;
    Ok(ER::Null)
}

fn eval_function_declaration<'a>(
    declaration: &Box<'_, Function<'a>>,
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    unsafe {
        let function = ptr::read(declaration).unbox();
        if let Some(id) = function.id {
            if let Some(body) = function.body {
                let params = function.params.unbox().items;
                let body = body.unbox().statements;
                let env = Rc::clone(&environment);
                let function = ER::Function(params, body, env);
                environment
                    .borrow_mut()
                    .define(id.name.to_owned(), function);
            }
        }
    }
    Ok(ER::Null)
}

fn eval_variable_declarator<'a>(
    declarators: &Vec<'_, VariableDeclarator>,
    environment: Rc<RefCell<Environment>>,
) -> Result<ER<'a>> {
    for declarator in declarators {
        if declarator.init.is_none() {
            continue;
        }
        match &declarator.id.kind {
            BindingPatternKind::BindingIdentifier(identifier) => {
                let value =
                    eval_expression(declarator.init.as_ref().unwrap(), Rc::clone(&environment))?;
                environment
                    .borrow_mut()
                    .define(identifier.name.to_owned(), value);
            }
        }
    }
    Ok(ER::Null)
}

fn eval_expression_statement<'a>(
    expression_stmt: &Box<'_, ExpressionStatement>,
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    eval_expression(&expression_stmt.expression, environment)
}

fn eval_expression<'a>(
    expression: &Expression<'_>,
    environment: Rc<RefCell<Environment<'a>>>,
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
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    environment
        .borrow()
        .get(expression.name.to_owned(), expression.span)
        .map(|v| v.clone())
}

fn eval_array_expression<'a>(
    expression: &Box<'_, ArrayExpression>,
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    let mut result = StdVec::new();
    for element in &expression.elements {
        let value = eval_array_expression_element(element, Rc::clone(&environment))?;
        result.push(value);
    }
    Ok(ER::Array(result))
}

fn eval_array_expression_element<'a>(
    expression: &ArrayExpressionElement<'_>,
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    match expression {
        ArrayExpressionElement::Expression(expression) => {
            eval_expression(expression, Rc::clone(&environment))
        }
    }
}

fn eval_binary_expression<'a>(
    expression: &BinaryExpression<'_>,
    environment: Rc<RefCell<Environment>>,
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
    environment: Rc<RefCell<Environment>>,
    operator: &BinaryOperator,
) -> Result<ER<'a>> {
    let left_eval = eval_expression(left, Rc::clone(&environment))?;
    let right_eval = eval_expression(right, Rc::clone(&environment))?;

    match (left_eval, right_eval) {
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
    environment: Rc<RefCell<Environment>>,
    operator: &BinaryOperator,
) -> Result<ER<'a>> {
    let left_eval = eval_expression(left, Rc::clone(&environment))?;
    let right_eval = eval_expression(right, Rc::clone(&environment))?;

    match (left_eval, right_eval) {
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
    environment: Rc<RefCell<Environment>>,
    operator: &BinaryOperator,
) -> Result<ER<'a>> {
    let left_eval = eval_expression(left, Rc::clone(&environment))?;
    let right_eval = eval_expression(right, Rc::clone(&environment))?;

    match (left_eval, right_eval) {
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
    environment: Rc<RefCell<Environment>>,
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
    environment: Rc<RefCell<Environment>>,
    operator: &LogicalOperator,
) -> Result<ER<'a>> {
    let left_eval = eval_expression(left, Rc::clone(&environment))?;
    let right_eval = eval_expression(right, Rc::clone(&environment))?;

    match (left_eval, right_eval) {
        (ER::Boolean(left), ER::Boolean(right)) => match operator {
            LogicalOperator::Or => Ok(ER::Boolean(left || right)),
            LogicalOperator::And => Ok(ER::Boolean(left && right)),
        },
        _ => Err(diagnostics::InvalidBoolean(left.span().merge(&right.span())).into()),
    }
}

fn eval_if_statement<'a>(
    if_stmt: &Box<'_, IfStatement<'a>>,
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    let test = eval_expression(&if_stmt.test, Rc::clone(&environment))?;
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
    environment: Rc<RefCell<Environment<'a>>>,
) -> Result<ER<'a>> {
    match &expression.callee {
        Expression::Identifier(identifier) => {
            let function = environment
                .borrow()
                .get(identifier.name.to_owned(), identifier.span)?;
            let mut arguments = vec![];
            for arg in &expression.arguments {
                match arg {
                    Argument::Expression(expression) => {
                        arguments.push(eval_expression(expression, Rc::clone(&environment))?);
                    }
                }
            }
            apply_function(function, arguments, expression.span)
        }
        _ => unreachable!(),
    }
}

fn apply_function<'a>(
    function: ER<'a>,
    arguments: StdVec<ER<'a>>,
    callee_span: Span,
) -> Result<ER<'a>> {
    match function {
        ER::Function(params, body, env) => {
            let env = Rc::new(RefCell::new(Environment::extend(env)));
            if params.len() != arguments.len() {
                Err(diagnostics::InvalidNumberOfArguments(callee_span).into())
            } else {
                for (param, arg) in params.iter().zip(arguments) {
                    let param_name = get_atom_formal_parameters(param);
                    env.borrow_mut().define(param_name, arg);
                }
                eval_block_body(&body, env)
            }
        }
        _ => unreachable!(),
    }
}

fn get_atom_formal_parameters(param: &FormalParameter) -> Atom {
    match &param.pattern.kind {
        BindingPatternKind::BindingIdentifier(identifier) => identifier.name.to_owned(),
    }
}
