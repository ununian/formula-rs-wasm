use core::fmt::Display;

use alloc::{format, string::ToString};
use num::{Rational64, ToPrimitive, Zero};

use super::error::ExecuteError;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Value<'a> {
    Error(ExecuteError), // TODO DELETE
    Bool(bool),
    Number(Rational64),
    String(&'a str),
    DateTime(u64),
    Duration(i64),
    // Array(Vec<Value>),
    // Object(Object),
    // Function(Function),
}

impl Value<'_> {
    pub fn add(self, _rhs: Value) -> Value {
        match (self.clone(), _rhs.clone()) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            // (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b).as_str()),

            // (Value::Number(a), Value::String(b)) => Value::String(format!("{}{}", a, b).as_str()),
            // (Value::String(a), Value::Number(b)) => Value::String(format!("{}{}", a, b).as_str()),

            (Value::DateTime(a), Value::Duration(b)) => Value::DateTime(a + b.to_u64().unwrap()),
            (Value::Duration(a), Value::DateTime(b)) => Value::DateTime(a.to_u64().unwrap() + b),
            (Value::Duration(a), Value::Duration(b)) => Value::Duration(a + b),

            _ => Value::Error(ExecuteError::operator_mismatch(
                "+",
                self.to_string().as_str(),
                Some(self.to_string().as_str()),
            )),
        }
    }

    pub fn sub(self, _rhs: Value) -> Value {
        match (&self, &_rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),

            (Value::DateTime(a), Value::Duration(b)) => Value::DateTime(a - b.to_u64().unwrap()),
            (Value::Duration(a), Value::Duration(b)) => Value::Duration(a - b),
            _ => Value::Error(ExecuteError::operator_mismatch(
                "-",
                self.to_string().as_str(),
                Some(self.to_string().as_str()),
            )),
        }
    }

    pub fn mul(self, _rhs: Value) -> Value {
        match (&self, &_rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::Duration(a), Value::Number(b)) => Value::Duration(
                (a.to_f64().unwrap() * b.to_f64().unwrap())
                    .floor()
                    .to_i64()
                    .unwrap(),
            ),
            _ => Value::Error(ExecuteError::operator_mismatch(
                "*",
                self.to_string().as_str(),
                Some(self.to_string().as_str()),
            )),
        }
    }

    pub fn div(self, _rhs: Value) -> Value {
        match (&self, &_rhs) {
            (Value::Number(a), Value::Number(b)) => {
                if b == &Rational64::zero() {
                    Value::Error(ExecuteError::divide_by_zero())
                } else {
                    Value::Number(a / b)
                }
            }

            (Value::Duration(a), Value::Number(b)) => {
                if b == &Rational64::zero() {
                    Value::Error(ExecuteError::divide_by_zero())
                } else {
                    Value::Duration(
                        (a.to_f64().unwrap() / b.to_f64().unwrap())
                            .floor()
                            .to_i64()
                            .unwrap(),
                    )
                }
            }

            _ => Value::Error(ExecuteError::operator_mismatch(
                "/",
                self.to_string().as_str(),
                Some(self.to_string().as_str()),
            )),
        }
    }

    pub fn factorial(self) -> Value<'static> {
        match self {
            Value::Number(a) => {
                if a.is_integer() {
                    if a.lt(&Rational64::zero()) {
                        return Value::Error(ExecuteError::factorial_not_negative());
                    }

                    let mut result = Rational64::from_integer(1);
                    for i in 1..a.to_i64().unwrap() + 1 {
                        result *= Rational64::from_integer(i);
                    }
                    Value::Number(result)
                } else {
                    Value::Error(ExecuteError::factorial_not_integer())
                }
            }
            _ => Value::Error(ExecuteError::operator_mismatch(
                "Factorial",
                self.to_string().as_str(),
                None,
            )),
        }
    }

    pub fn pow(self, _rhs: Value) -> Value {
        match (&self, &_rhs) {
            (Value::Number(a), Value::Number(b)) => {
                if !b.is_integer() {
                    return Value::Error(ExecuteError::pow_not_rational());
                }
                match b.to_i32() {
                    Some(power) => Value::Number(a.pow(power)),
                    None => Value::Error(ExecuteError::number_conversion_error()),
                }
            }
            _ => Value::Error(ExecuteError::operator_mismatch(
                "Pow",
                self.to_string().as_str(),
                Some(self.to_string().as_str()),
            )),
        }
    }

    pub fn modulo(self, _rhs: Value) -> Value {
        match (&self, &_rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => Value::Error(ExecuteError::operator_mismatch(
                "FormulaOperator::Modulo",
                self.to_string().as_str(),
                Some(self.to_string().as_str()),
            )),
        }
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::Error(err) => write!(f, "Error: {:?}", err),
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
            // Value::Array(a) => {
            //     write!(f, "[")?;
            //     for (i, v) in a.iter().enumerate() {
            //         write!(f, "{}", v)?;
            //         if i != a.len() - 1 {
            //             write!(f, ", ")?;
            //         }
            //     }
            //     write!(f, "]")
            // }
            Value::DateTime(_) => write!(f, "{:?}", self),
            Value::Duration(_) => write!(f, "{:?}", self),
        }
    }
}
