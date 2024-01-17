use crate::{
    diagnostics,
    eval::{Eval, ER},
};
use wave_ast::ast::Expression;
use wave_diagnostics::{Error, Result};
use wave_span::GetSpan;

pub struct ArithmeticWrapper<'a, T>(pub &'a T);

type Number = f64;

impl ArithmeticWrapper<'_, Expression<'_>> {
    pub fn add_e(&self, rhs: Self) -> Result<Number> {
        Ok(self.get_number()? + (rhs.get_number()?))
    }

    pub fn sub_e(&self, rhs: Self) -> Result<Number> {
        Ok(self.get_number()? - (rhs.get_number()?))
    }

    pub fn mul_e(&self, rhs: Self) -> Result<Number> {
        Ok(self.get_number()? * (rhs.get_number()?))
    }

    pub fn div_e(&self, rhs: Self) -> Result<Number> {
        Ok(self.get_number()? / (rhs.get_number()?))
    }

    pub fn pow_e(&self, rhs: Self) -> Result<Number> {
        Ok(self.get_number()?.powf(rhs.get_number()?))
    }

    pub fn rem_e(&self, rhs: Self) -> Result<Number> {
        Ok(self.get_number()? % (rhs.get_number()?))
    }

    pub fn b_or(&self, rhs: Self) -> Result<Number> {
        Ok((self.get_number()? as i64 | (rhs.get_number()? as i64)) as f64)
    }

    pub fn b_and(&self, rhs: Self) -> Result<Number> {
        Ok((self.get_number()? as i64 & (rhs.get_number()? as i64)) as f64)
    }

    pub fn b_xor(&self, rhs: Self) -> Result<Number> {
        Ok((self.get_number()? as i64 ^ (rhs.get_number()? as i64)) as f64)
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
