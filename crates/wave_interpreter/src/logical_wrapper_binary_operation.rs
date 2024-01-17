use crate::{
    diagnostics,
    eval::{Eval, ER},
};
use wave_ast::ast::Expression;
use wave_diagnostics::{Error, Result};
use wave_span::GetSpan;

pub struct LogicalWrapper<'a, T>(pub &'a T);

type Boolean = bool;
type Number = f64;

impl LogicalWrapper<'_, Expression<'_>> {
    pub fn lt(&self, rhs: Self) -> Result<Boolean> {
        Ok(self.get_number()? < (rhs.get_number()?))
    }

    pub fn lt_e(&self, rhs: Self) -> Result<Boolean> {
        Ok(self.get_number()? <= (rhs.get_number()?))
    }

    pub fn gt(&self, rhs: Self) -> Result<Boolean> {
        Ok(self.get_number()? > (rhs.get_number()?))
    }

    pub fn gt_e(&self, rhs: Self) -> Result<Boolean> {
        Ok(self.get_number()? >= (rhs.get_number()?))
    }

    pub fn eq(&self, rhs: Self) -> Result<Boolean> {
        Ok(self.0.eval()? == rhs.0.eval()?)
    }

    pub fn ne(&self, rhs: Self) -> Result<Boolean> {
        Ok(self.0.eval()? != rhs.0.eval()?)
    }

    pub fn or(&self, rhs: Self) -> Result<Boolean> {
        Ok(self.get_bool()? || (rhs.get_bool()?))
    }

    pub fn and(&self, rhs: Self) -> Result<Boolean> {
        Ok(self.get_bool()? && (rhs.get_bool()?))
    }

    pub fn get_bool(&self) -> Result<Boolean> {
        match self.0 {
            Expression::BooleanLiteral(lit) => Ok(lit.value),
            Expression::BinaryExpression(expr) => match expr.eval()? {
                ER::Boolean(bool) => Ok(bool),
                _ => Err(self.not_a_boolean_error()),
            },
            _ => Err(self.not_a_boolean_error()),
        }
    }

    pub fn not_a_boolean_error(&self) -> Error {
        let expression = &self.0;
        diagnostics::InvalidNumber(expression.span()).into()
    }

    fn get_number(&self) -> Result<Number> {
        match self.0 {
            Expression::NumberLiteral(lit) => Ok(lit.value),
            Expression::BinaryExpression(expr) => match expr.eval()? {
                ER::Number(num) => Ok(num),
                _ => Err(self.not_a_number_error()),
            },
            _ => Err(self.not_a_number_error()),
        }
    }

    fn not_a_number_error(&self) -> Error {
        let expression = &self.0;
        diagnostics::InvalidNumber(expression.span()).into()
    }
}
