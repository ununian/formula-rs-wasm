use alloc::string::String;

use crate::types::{operator::FormulaOperator, types::FormulaValueType};

// 数值相关错误
#[derive(Clone, Debug, PartialEq)]
pub enum ExecuteErrorType {
    UnknownError,

    // 操作符错误
    OperatorMismatchError(FormulaOperator, FormulaValueType, Option<FormulaValueType>),

    DivideByZero,
    NumberConversionError,
    PowNotRational,
    FactorialNotInteger,
    FactorialNotNegative,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteError {
    pub type_: ExecuteErrorType,
    pub message: Option<String>,
}

impl ExecuteError {
    pub fn new(type_: ExecuteErrorType) -> Self {
        Self {
            type_,
            message: None,
        }
    }

    pub fn unknown() -> Self {
        Self::new(ExecuteErrorType::UnknownError)
    }

    pub fn with_message(self, msg: String) -> Self {
        Self {
            type_: self.type_,
            message: Some(msg),
        }
    }

    pub fn operator_mismatch(
        operator: FormulaOperator,
        lhs: FormulaValueType,
        rhs: Option<FormulaValueType>,
    ) -> Self {
        Self::new(ExecuteErrorType::OperatorMismatchError(operator, lhs, rhs))
    }

    pub fn divide_by_zero() -> Self {
        Self::new(ExecuteErrorType::DivideByZero)
    }

    pub fn number_conversion_error() -> Self {
        Self::new(ExecuteErrorType::NumberConversionError)
    }

    pub fn pow_not_rational() -> Self {
        Self::new(ExecuteErrorType::PowNotRational)
    }

    pub fn factorial_not_integer() -> Self {
        Self::new(ExecuteErrorType::FactorialNotInteger)
    }

    pub fn factorial_not_negative() -> Self {
        Self::new(ExecuteErrorType::FactorialNotNegative)
    }
}
