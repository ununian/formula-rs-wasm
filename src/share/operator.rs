use alloc::string::String;
use num::Rational64;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OperatorCode {
    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Factorial, // 阶乘
    Power,     // 幂

    // Push 立即数入栈
    PushNumber(Rational64),
    PushString(String),

    // Load 标识符入栈
    LoadIdentifier(String),

    // func
    Call(u8),
}
