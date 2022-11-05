use super::{error::FormulaError, value::FormulaValue};
use crate::alloc::string::ToString;
use alloc::vec;

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaOperator {
    // 数值计算
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Factorial,
    Rem,
    Neg,

    // 逻辑计算
    And,
    Or,
    Not,

    // 比较计算
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,

    // 其他
    Dot,
    Call,
}

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

impl From<FormulaValue> for FormulaValueType {
    fn from(value: FormulaValue) -> Self {
        match value {
            FormulaValue::Error(_) => FormulaValueType::Error,
            FormulaValue::Bool(_) => FormulaValueType::Bool,
            FormulaValue::Number(_) => FormulaValueType::Number,
            FormulaValue::String(_) => FormulaValueType::String,
            FormulaValue::DateTime(_) => FormulaValueType::DateTime,
            FormulaValue::Duration(_) => FormulaValueType::Duration,
            FormulaValue::Array(_) => FormulaValueType::Array,
        }
    }
}

impl From<FormulaValueType> for FormulaValue {
    fn from(value: FormulaValueType) -> Self {
        match value {
            FormulaValueType::Error => FormulaValue::Error(FormulaError::default_error()),
            FormulaValueType::Bool => FormulaValue::Bool(false),
            FormulaValueType::Number => FormulaValue::Number(0.into()),
            FormulaValueType::String => FormulaValue::String("".to_string()),
            FormulaValueType::DateTime => FormulaValue::DateTime(0),
            FormulaValueType::Duration => FormulaValue::Duration(0),
            FormulaValueType::Array => FormulaValue::Array(vec![]),
        }
    }
}
