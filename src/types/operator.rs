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
