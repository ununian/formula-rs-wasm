use alloc::string::String;

use super::types::{FormulaOperator, FormulaValueType};

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaErrorType {
    DefaultError,
    
    // 类型相关错误
    TypeMismatchError,

    // 数值相关错误
    OperatorMismatchError(FormulaOperator, FormulaValueType, Option<FormulaValueType>),
    DivideByZero,
    NumberConversionError,
    PowNotRational,
    FactorialNotInteger,
    FactorialNotNegative,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaError {
    pub type_: FormulaErrorType,
    pub message: Option<String>,
}

impl FormulaError {
    pub fn new(type_: FormulaErrorType) -> Self {
        Self {
            type_,
            message: None,
        }
    }

    pub fn default_error() -> Self {
        Self::new(FormulaErrorType::DefaultError)
    }

    pub fn with_message(self, msg: String) -> Self {
        Self {
            type_: self.type_,
            message: Some(msg),
        }
    }

    pub fn operator_mismatch_error(
        operator: FormulaOperator,
        lhs: FormulaValueType,
        rhs: Option<FormulaValueType>,
    ) -> Self {
        Self::new(FormulaErrorType::OperatorMismatchError(operator, lhs, rhs))
    }

    pub fn divide_by_zero() -> Self {
        Self::new(FormulaErrorType::DivideByZero)
    }

    pub fn number_conversion_error() -> Self {
        Self::new(FormulaErrorType::NumberConversionError)
    }

    pub fn pow_not_rational() -> Self {
        Self::new(FormulaErrorType::PowNotRational)
    }

    pub fn factorial_not_integer() -> Self {
        Self::new(FormulaErrorType::FactorialNotInteger)
    }

    pub fn factorial_not_negative() -> Self {
        Self::new(FormulaErrorType::FactorialNotNegative)
    }

    pub fn type_mismatch_error() -> Self {
        Self::new(FormulaErrorType::TypeMismatchError)
    }
}
