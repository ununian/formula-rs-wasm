use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use super::value::Value;
use crate::share::operator::OperatorCode;

pub struct Runner;

impl Runner {
    pub fn run(&self, operators: Vec<OperatorCode>) -> Result<Value, String> {
        let mut stack: Vec<Value> = vec![];

        for operator in operators {
            operator.run(&mut stack)?;
        }

        match stack.len() {
            0 => return Err("stack is empty".to_string()),
            1 => Ok(stack.pop().unwrap()),
            _ => return Err("stack is not empty".to_string()),
        }
    }
}

pub trait Runnable {
    fn run(&self, stack: &mut Vec<Value>) -> Result<(), String>;
}

impl Runnable for OperatorCode<'_> {
    fn run(&self, stack: &mut Vec<Value>) -> Result<(), String> {
        match self {
            OperatorCode::Add => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.add(rhs));
            }
            OperatorCode::Subtract => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.sub(rhs));
            }
            OperatorCode::Multiply => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.mul(rhs));
            }
            OperatorCode::Divide => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.div(rhs));
            }
            OperatorCode::Modulo => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.rem(rhs));
            }
            OperatorCode::Factorial => {
                let lhs = stack.pop().unwrap();
                stack.push(lhs.factorial());
            }

            OperatorCode::PushNumber(val) => {
                stack.push(Value::Number(*val));
            }
            OperatorCode::PushString(_) => todo!(),
            OperatorCode::LoadIdentifier(_) => todo!(),
            OperatorCode::StoreIdentifier => todo!(),
            OperatorCode::Call => todo!(),
        }

        Ok(())
    }
}
