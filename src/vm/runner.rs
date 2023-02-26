use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use hashbrown::HashMap;
use num::{FromPrimitive, Rational64};

use super::{error::ExecuteError, value::Value};
use crate::share::operator::OperatorCode;

pub struct Runner;

impl Runner {
    pub fn run(
        &self,
        operators: Vec<OperatorCode>,
        heap: &mut HashMap<String, Value>,
    ) -> Result<Value, ExecuteError> {
        let mut stack: Vec<Value> = vec![];

        for operator in operators {
            operator.run(&mut stack, heap)?;
        }

        // heap.insert(
        //     "abc".to_string(),
        //     Value::Number(Rational64::from_f64(12.0).unwrap()),
        // );

        match stack.len() {
            0 => return Err(ExecuteError::result_count_mismatch(0)),
            1 => Ok(stack.pop().unwrap()),
            count => return Err(ExecuteError::result_count_mismatch(count)),
        }
    }
}

pub trait Runnable {
    fn run(
        &self,
        stack: &mut Vec<Value>,
        heap: &mut HashMap<String, Value>,
    ) -> Result<(), ExecuteError>;
}

impl Runnable for OperatorCode {
    fn run(
        &self,
        stack: &mut Vec<Value>,
        heap: &mut HashMap<String, Value>,
    ) -> Result<(), ExecuteError> {
        match self {
            OperatorCode::Add => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.add(rhs)?);
            }
            OperatorCode::Subtract => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.sub(rhs)?);
            }
            OperatorCode::Multiply => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.mul(rhs)?);
            }
            OperatorCode::Divide => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.div(rhs)?);
            }
            OperatorCode::Modulo => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.modulo(rhs)?);
            }
            OperatorCode::Factorial => {
                let lhs = stack.pop().unwrap();
                stack.push(lhs.factorial()?);
            }
            OperatorCode::Power => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                stack.push(lhs.pow(rhs)?);
            }

            OperatorCode::PushNumber(val) => {
                stack.push(Value::Number(*val));
            }

            OperatorCode::PushString(val) => {
                stack.push(Value::String(val.clone()));
            }
            OperatorCode::LoadIdentifier(name) => {
                let val = heap.get(name);
                match val {
                    Some(val) => stack.push(val.clone()),
                    None => return Err(ExecuteError::identifier_not_found(name)),
                }
            },
            OperatorCode::Call => todo!(),
        }

        Ok(())
    }
}
