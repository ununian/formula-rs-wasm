use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

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

    IdentifierNotFound,

    // Runtime 错误
    StackNotEmpty, // 程序一开始的栈不为空
    NotAFunction,
    FunctionNotFound,
    FunctionInvalidArgument,

    DotInputNotAObjectArray,
    DotNotFountProperty,
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

    pub fn operator_mismatch(operator: String, lhs: String, rhs: Option<String>) -> Self {
        Self::new(ExecuteErrorType::OperatorMismatchError).with_message(format!(
            "{} {} {}",
            lhs,
            operator,
            rhs.unwrap_or("".to_string())
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
            .with_message(format!("result count expect 1, actual: {}", actual))
    }

    pub fn identifier_not_found(identifier: &String) -> Self {
        Self::new(ExecuteErrorType::IdentifierNotFound)
            .with_message(format!("identifier not found: {}", identifier))
    }

    pub fn stack_not_empty() -> Self {
        Self::new(ExecuteErrorType::StackNotEmpty)
    }

    pub fn not_a_function() -> Self {
        Self::new(ExecuteErrorType::NotAFunction)
    }

    pub fn function_not_found(name: &String) -> Self {
        Self::new(ExecuteErrorType::FunctionNotFound)
            .with_message(format!("function not found: {}", name))
    }

    pub fn function_invalid_argument(except: Vec<&str>, actual: Vec<&str>) -> Self {
        Self::new(ExecuteErrorType::FunctionInvalidArgument).with_message(format!(
            "function invalid argument, except: {:?}, actual: {:?}",
            except.join(","),
            actual.join(",")
        ))
    }

    pub fn dot_input_not_object_array() -> Self {
        Self::new(ExecuteErrorType::DotInputNotAObjectArray)
    }

    pub fn dot_not_found_property() -> Self {
        Self::new(ExecuteErrorType::DotNotFountProperty)
    }
}
