use alloc::{string::String, vec, vec::Vec};

use super::{error::ExecuteError, value::Value};

pub trait RuntimeFunction {
    fn run(&self, args: &Vec<Value>) -> Result<Value, ExecuteError>;
}
pub struct SumFunction;

impl RuntimeFunction for SumFunction {
    fn run(&self, args: &Vec<Value>) -> Result<Value, ExecuteError> {
        if args.iter().all(|arg| arg.is_number()) {
            let mut sum = Value::Number(0.into());
            for arg in args {
                sum = sum.add(arg.clone())?;
            }
            return Ok(sum);
        }

        match args[0] {
            Value::Array(ref a) => {
                let mut sum = Value::Number(0.into());
                for arg in a {
                    sum = sum.add(arg.clone())?;
                }
                return Ok(sum);
            }
            _ => Err(ExecuteError::function_invalid_argument(
                vec!["Number", "Number[]"],
                args.iter().map(|a| a.get_type()).collect(),
            )),
        }
    }
}

pub struct CountFunction;

impl RuntimeFunction for CountFunction {
    fn run(&self, args: &Vec<Value>) -> Result<Value, ExecuteError> {
        Ok(Value::Number((args.len() as i64).into()))
    }
}

pub fn run_runtime_function(name: &String, args: &Vec<Value>) -> Result<Value, ExecuteError> {
    match name.as_str() {
        "sum" => SumFunction.run(args),
        "count" => CountFunction.run(args),
        _ => Err(ExecuteError::function_not_found(&name)),
    }
}
