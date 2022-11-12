use super::{error::TypeError, operator::FormulaOperator};

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaValueType {
    Error,
    Bool,
    Number,
    String,
    DateTime,
    Duration,
    Array,
}

impl FormulaValueType {
    pub fn add(self, _rhs: FormulaValueType) -> Result<FormulaValueType, TypeError> {
        match (self.clone(), _rhs.clone()) {
            (FormulaValueType::Number, FormulaValueType::Number) => Ok(FormulaValueType::Number),
            (FormulaValueType::String, FormulaValueType::String) => Ok(FormulaValueType::String),

            (FormulaValueType::Number, FormulaValueType::String) => Ok(FormulaValueType::String),
            (FormulaValueType::String, FormulaValueType::Number) => Ok(FormulaValueType::String),

            (FormulaValueType::DateTime, FormulaValueType::Duration) => {
                Ok(FormulaValueType::DateTime)
            }
            (FormulaValueType::Duration, FormulaValueType::DateTime) => {
                Ok(FormulaValueType::DateTime)
            }
            (FormulaValueType::Duration, FormulaValueType::Duration) => {
                Ok(FormulaValueType::Duration)
            }

            _ => Err(TypeError::operator_mismatch(
                FormulaOperator::Add,
                self,
                Some(_rhs),
            )),
        }
    }
}
