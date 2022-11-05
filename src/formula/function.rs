use alloc::vec::Vec;

use super::{identifier::Identifier, value::FormulaValue};

pub trait Callable {
    fn call(&self, args: Vec<FormulaValue>) -> FormulaValue;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Identifier,
    pub args: Vec<FunctionArgument>,
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