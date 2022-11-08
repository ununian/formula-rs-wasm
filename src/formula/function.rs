use alloc::vec::Vec;

use super::{identifier::Identifier, value::FormulaValue};

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Identifier,
    pub arguments: Vec<FunctionArgument>, // 实参
}

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionArgument {
    Value(FormulaValue),
    Identifier(Identifier),
    Function(Function),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompareFunction {

}