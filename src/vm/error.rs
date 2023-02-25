use alloc::format;

// 数值相关错误
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ExecuteErrorType {
    UnknownError,

    // 操作符错误
    OperatorMismatchError,

    DivideByZero,
    NumberConversionError,
    PowNotRational,
    FactorialNotInteger,
    FactorialNotNegative,
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
}
