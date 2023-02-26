use alloc::vec::Vec;

use super::{
    context::RuntimeContext, error::ExecuteError, function::run_runtime_function, value::Value,
};
use crate::share::operator::OperatorCode;

pub struct Runner;

impl Runner {
    pub fn run(
        &self,
        operators: Vec<OperatorCode>,
        context: &mut RuntimeContext,
    ) -> Result<Value, ExecuteError> {
        if context.value_stack.len() > 0 {
            return Err(ExecuteError::stack_not_empty());
        }

        for operator in operators {
            operator.run(context)?;
        }

        // heap.insert(
        //     "abc".to_string(),
        //     Value::Number(Rational64::from_f64(12.0).unwrap()),
        // );

        match context.value_stack.len() {
            0 => return Err(ExecuteError::result_count_mismatch(0)),
            1 => Ok(context.value_stack.pop().unwrap()),
            count => return Err(ExecuteError::result_count_mismatch(count)),
        }
    }
}

pub trait Runnable {
    fn run(&self, context: &mut RuntimeContext) -> Result<(), ExecuteError>;
}

impl Runnable for OperatorCode {
    fn run(&self, ctx: &mut RuntimeContext) -> Result<(), ExecuteError> {
        match self {
            OperatorCode::Add => {
                let rhs = ctx.value_stack.pop().unwrap();
                let lhs = ctx.value_stack.pop().unwrap();

                ctx.value_stack.push(lhs.add(rhs)?);
            }
            OperatorCode::Subtract => {
                let rhs = ctx.value_stack.pop().unwrap();
                let lhs = ctx.value_stack.pop().unwrap();

                ctx.value_stack.push(lhs.sub(rhs)?);
            }
            OperatorCode::Multiply => {
                let rhs = ctx.value_stack.pop().unwrap();
                let lhs = ctx.value_stack.pop().unwrap();

                ctx.value_stack.push(lhs.mul(rhs)?);
            }
            OperatorCode::Divide => {
                let rhs = ctx.value_stack.pop().unwrap();
                let lhs = ctx.value_stack.pop().unwrap();

                ctx.value_stack.push(lhs.div(rhs)?);
            }
            OperatorCode::Modulo => {
                let rhs = ctx.value_stack.pop().unwrap();
                let lhs = ctx.value_stack.pop().unwrap();

                ctx.value_stack.push(lhs.modulo(rhs)?);
            }
            OperatorCode::Factorial => {
                let lhs = ctx.value_stack.pop().unwrap();
                ctx.value_stack.push(lhs.factorial()?);
            }
            OperatorCode::Power => {
                let rhs = ctx.value_stack.pop().unwrap();
                let lhs = ctx.value_stack.pop().unwrap();

                ctx.value_stack.push(lhs.pow(rhs)?);
            }

            OperatorCode::PushNumber(val) => {
                ctx.value_stack.push(Value::Number(*val));
            }

            OperatorCode::PushString(val) => {
                ctx.value_stack.push(Value::String(val.clone()));
            }
            OperatorCode::LoadIdentifier(name) => {
                let val = ctx.heap.get(name);
                match val {
                    Some(val) => ctx.value_stack.push(val.clone()),
                    None => return Err(ExecuteError::identifier_not_found(name)),
                }
            }
            OperatorCode::Call(arg_count) => {
                let mut args = Vec::new();
                for _ in 0..*arg_count {
                    args.push(ctx.value_stack.pop().unwrap());
                }
                let func = ctx.value_stack.pop().unwrap();
                match func {
                    Value::Function(name) => {
                        let result = run_runtime_function(&name, &args)?;
                        ctx.value_stack.push(result);
                    }
                    _ => return Err(ExecuteError::not_a_function()),
                }
            }
            OperatorCode::LoadPropertyAccess(property) => {
                let val = ctx.value_stack.pop().unwrap();
                match val {
                    Value::Array(arr) => {
                        if arr.len() == 0 { // 空数组
                            ctx.value_stack.push(Value::Array(Vec::new()));
                            return Ok(());
                        }

                        let mut result = Vec::new();
                        for item in arr {
                            match item {
                                Value::Object(obj) => {
                                    let val = obj.get(property);
                                    match val {
                                        Some(val) => result.push(val.clone()),
                                        None => return Err(ExecuteError::dot_input_not_object_array()),
                                    }
                                }
                                _ => return Err(ExecuteError::dot_input_not_object_array()),
                            }
                        }

                        ctx.value_stack.push(Value::Array(result));
                    }
                    _ => return Err(ExecuteError::dot_input_not_object_array()),
                }
            }
        }

        Ok(())
    }
}
