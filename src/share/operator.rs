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
    
    // 入栈
    LoadIdentifier(String), // Load 标识符
    LoadPropertyAccess(String), // 用来访问对象的属性、函数等, 但是目前的含义是 map 功能，例如 subtask.estimatePoint -> subtask.map(task => task.estimatePoint)

    // func
    Call(u8),
}
