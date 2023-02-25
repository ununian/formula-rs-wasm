use num::Rational64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatorCode<'a> {
    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Factorial, // 阶乘

    // Push 常量入栈
    PushNumber(Rational64),
    PushString(&'a str),

    // Load 标识符入栈
    LoadIdentifier(&'a str),

    // Store 栈顶元素出栈并赋值给标识符
    StoreIdentifier,

    // func
    Call,
}
