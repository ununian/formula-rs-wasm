use alloc::{format, string::String};

// 数值相关错误
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ExecuteErrorType {
    UnknownError,

    ResultCountMismatchError, // 结果数量不匹配
    


    // 操作符错误
    OperatorMismatchError,

    DivideByZero,
    NumberConversionError,
    PowNotRational,
    FactorialNotInteger,
    FactorialNotNegative,

    IdentifierNotFound
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct ExecuteError {
    pub type_: ExecuteErrorType,
    pub message: Option<&'static str>,
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

    pub fn with_message(self, _: &str) -> Self {
        Self {
            type_: self.type_,
            message: Some("msg"), // TODO
        }
    }

    pub fn operator_mismatch(operator: &str, lhs: &str, rhs: Option<&str>) -> Self {
        Self::new(ExecuteErrorType::OperatorMismatchError).with_message(&format!(
            "{} {} {}",
            lhs,
            operator,
            rhs.unwrap_or("")
        ))
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

    pub fn result_count_mismatch(actual: usize) -> Self {
        Self::new(ExecuteErrorType::ResultCountMismatchError)
            .with_message(&format!("result count expect 1, actual: {}", actual))
    }

    pub fn identifier_not_found(identifier: &String) -> Self {
        Self::new(ExecuteErrorType::IdentifierNotFound)
            .with_message(&format!("identifier not found: {}", identifier))
    }
}
