use core::fmt::Display;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use hashbrown::HashMap;
use num::{Rational64, ToPrimitive, Zero};

use super::error::ExecuteError;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(Rational64),
    String(String),
    DateTime(u64),
    Duration(i64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Function(String),

    Null,
}

impl Value {
    pub fn add(self, rhs: Value) -> Result<Value, ExecuteError> {
        match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + &b)),

            (Value::Number(a), Value::String(b)) => Ok(Value::String(a.to_string() + &b)),
            (Value::String(a), Value::Number(b)) => Ok(Value::String(a.clone() + &b.to_string())),

            (Value::DateTime(a), Value::Duration(b)) => {
                Ok(Value::DateTime(a + b.to_u64().unwrap()))
            }
            (Value::Duration(a), Value::DateTime(b)) => {
                Ok(Value::DateTime(a.to_u64().unwrap() + b))
            }
            (Value::Duration(a), Value::Duration(b)) => Ok(Value::Duration(a + b)),

            _ => Err(ExecuteError::operator_mismatch(
                "+".to_string(),
                self.to_string(),
                Some(rhs.to_string()),
            )),
        }
    }

    pub fn sub(self, rhs: Value) -> Result<Value, ExecuteError> {
        match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),

            (Value::DateTime(a), Value::Duration(b)) => {
                Ok(Value::DateTime(a - b.to_u64().unwrap()))
            }
            (Value::Duration(a), Value::Duration(b)) => Ok(Value::Duration(a - b)),
            _ => Err(ExecuteError::operator_mismatch(
                "-".to_string(),
                self.to_string(),
                Some(rhs.to_string()),
            )),
        }
    }

    pub fn mul(self, rhs: Value) -> Result<Value, ExecuteError> {
        match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Duration(a), Value::Number(b)) => Ok(Value::Duration(
                (a.to_f64().unwrap() * b.to_f64().unwrap())
                    .floor()
                    .to_i64()
                    .unwrap(),
            )),
            (Value::Null, Value::Null) => Err(ExecuteError::operator_mismatch(
                "*".to_string(),
                self.to_string(),
                Some(self.to_string()),
            )),
            _ => Err(ExecuteError::operator_mismatch(
                "*".to_string(),
                self.to_string(),
                Some(rhs.to_string()),
            )),
        }
    }

    pub fn div(self, rhs: Value) -> Result<Value, ExecuteError> {
        match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => {
                if b == &Rational64::zero() {
                    Err(ExecuteError::divide_by_zero())
                } else {
                    Ok(Value::Number(a / b))
                }
            }

            (Value::Duration(a), Value::Number(b)) => {
                if b == &Rational64::zero() {
                    Err(ExecuteError::divide_by_zero())
                } else {
                    Ok(Value::Duration(
                        (a.to_f64().unwrap() / b.to_f64().unwrap())
                            .floor()
                            .to_i64()
                            .unwrap(),
                    ))
                }
            }

            _ => Err(ExecuteError::operator_mismatch(
                "/".to_string(),
                self.to_string(),
                Some(rhs.to_string()),
            )),
        }
    }

    pub fn factorial(self) -> Result<Value, ExecuteError> {
        match &self {
            Value::Number(a) => {
                if a.is_integer() {
                    if a.lt(&Rational64::zero()) {
                        return Err(ExecuteError::factorial_not_negative());
                    }

                    let mut result = Rational64::from_integer(1);
                    for i in 1..a.to_i64().unwrap() + 1 {
                        result *= Rational64::from_integer(i);
                    }
                    Ok(Value::Number(result))
                } else {
                    Err(ExecuteError::factorial_not_integer())
                }
            }
            _ => Err(ExecuteError::operator_mismatch(
                "Factorial".to_string(),
                self.to_string(),
                None,
            )),
        }
    }

    pub fn pow(self, rhs: Value) -> Result<Value, ExecuteError> {
        match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => {
                if !b.is_integer() {
                    return Err(ExecuteError::pow_not_rational());
                }
                match b.to_i32() {
                    Some(power) => Ok(Value::Number(a.pow(power))),
                    None => Err(ExecuteError::number_conversion_error()),
                }
            }
            _ => Err(ExecuteError::operator_mismatch(
                "Pow".to_string(),
                self.to_string(),
                Some(rhs.to_string()),
            )),
        }
    }

    pub fn modulo(self, rhs: Value) -> Result<Value, ExecuteError> {
        match (&self, &rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
            _ => Err(ExecuteError::operator_mismatch(
                "FormulaOperator::Modulo".to_string(),
                self.to_string(),
                Some(rhs.to_string()),
            )),
        }
    }

    pub fn compare(&self, op: &String, rhs: &Value) -> Result<bool, ExecuteError> {
        match (self, rhs) {
            (Value::String(lhs), Value::String(rhs)) => match op.as_str() {
                "=" | "==" => return Ok(lhs == rhs),
                "!=" => return Ok(lhs != rhs),
                _ => Err(ExecuteError::operator_mismatch(
                    op.clone(),
                    self.to_string(),
                    Some(rhs.to_string()),
                )),
            },
            (Value::Number(lhs), Value::Number(rhs)) => match op.as_str() {
                ">" => Ok(lhs.gt(&rhs)),
                ">=" => Ok(lhs.ge(&rhs)),
                "<" => Ok(lhs.lt(&rhs)),
                "<=" => Ok(lhs.le(&rhs)),
                "=" | "==" => Ok(lhs.eq(&rhs)),
                "!=" => Ok(lhs.ne(&rhs)),
                _ => Err(ExecuteError::operator_mismatch(
                    op.clone(),
                    self.to_string(),
                    Some(rhs.to_string()),
                )),
            },
            _ => Err(ExecuteError::operator_mismatch(
                op.clone(),
                self.to_string(),
                Some(rhs.to_string()),
            )),
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Value::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_date_time(&self) -> bool {
        match self {
            Value::DateTime(_) => true,
            _ => false,
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            Value::Array(_) => "Array",
            Value::Bool(_) => "Bool",
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::DateTime(_) => "DateTime",
            Value::Duration(_) => "Duration",
            Value::Function(_) => "Function",
            Value::Object(_) => "Object",
            Value::Null => "Null",
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => {
                if n.is_integer() {
                    write!(f, "{}", n.to_integer())
                } else if let Some(n) = n.to_f64() {
                    write!(f, "{}", n)
                } else {
                    write!(f, "Error")
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Array(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    write!(f, "{}", v)?;
                    if i != a.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Value::DateTime(_) => write!(f, "{:?}", self),
            Value::Duration(_) => write!(f, "{:?}", self),
            Value::Function(name) => write!(f, "Func {}()", name),
            Value::Object(_) => write!(f, "{:?}", self),
            Value::Null => write!(f, "null"),
        }
    }
}
