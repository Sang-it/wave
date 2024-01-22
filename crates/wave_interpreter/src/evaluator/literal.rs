use crate::evaluator::Primitive;
use crate::Runtime;
use wave_allocator::Box;
use wave_ast::{BooleanLiteral, NumberLiteral, StringLiteral};
use wave_diagnostics::Result;

impl<'a> Runtime<'a> {
    pub fn eval_boolean_literal(
        &self,
        expression: &Box<'_, BooleanLiteral>,
    ) -> Result<Primitive<'a>> {
        Ok(Primitive::Boolean(expression.value))
    }

    pub fn eval_number_literal(
        &self,
        expression: &Box<'_, NumberLiteral>,
    ) -> Result<Primitive<'a>> {
        Ok(Primitive::Number(expression.value))
    }

    pub fn eval_string_literal(
        &self,
        expression: &Box<'_, StringLiteral>,
    ) -> Result<Primitive<'a>> {
        Ok(Primitive::String(expression.value.to_string()))
    }
}
