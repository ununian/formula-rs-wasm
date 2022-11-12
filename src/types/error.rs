use alloc::string::String;

use super::{operator::FormulaOperator, types::FormulaValueType};

// 数值相关错误
#[derive(Clone, Debug, PartialEq)]
pub enum TypeErrorType {
    UnknownError,

    OperatorMismatchError(FormulaOperator, FormulaValueType, Option<FormulaValueType>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeError {
    pub type_: TypeErrorType,
    pub message: Option<String>,
}

impl TypeError {
    pub fn new(type_: TypeErrorType) -> Self {
        Self {
            type_,
            message: None,
        }
    }

    pub fn unknown() -> Self {
        Self::new(TypeErrorType::UnknownError)
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
        Self::new(TypeErrorType::OperatorMismatchError(operator, lhs, rhs))
    }
}
