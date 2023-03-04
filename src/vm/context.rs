use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use hashbrown::HashMap;

use super::{function::RuntimeFunction, value::Value};

pub struct RuntimeContext {
    pub heap: HashMap<String, Value>,
    pub function_table: HashMap<String, Box<dyn RuntimeFunction>>,
    pub value_stack: Vec<Value>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        RuntimeContext {
            heap: HashMap::new(),
            function_table: HashMap::new(),
            value_stack: Vec::new(),
        }
    }

    pub fn reset_stack(&mut self) {
        self.value_stack.clear();
    }

    pub fn inject_functions(&mut self) {
        self.set("SUM".to_string(), Value::Function("sum".to_string()));
        self.set("COUNT".to_string(), Value::Function("count".to_string()));
    }

    pub fn get(&self, key: &String) -> Option<&Value> {
        self.heap.get(key)
    }

    pub fn has(&self, key: &String) -> bool {
        self.heap.contains_key(key)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.heap.insert(key, value);
    }
}
