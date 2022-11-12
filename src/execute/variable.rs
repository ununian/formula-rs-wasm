use alloc::{
    string::{String, ToString},
    vec::Vec,
};


#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub name: Identifier,
    pub value: FormulaValueType,
    pub type_: FormulaValueType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableTable {
    pub variables: Vec<Variable>,
}

impl VariableTable {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, variable: Variable) {
        match self.variables.iter_mut().find(|v| v.name == variable.name) {
            Some(found) => {
                found.value = variable.value;
                found.type_ = variable.type_;
            }
            None => {
                self.variables.push(variable);
            }
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables
            .iter()
            .find(|v| v.name == Identifier(String::from(name)))
    }
}
