use core::fmt::Display;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use num::{Rational64, ToPrimitive, Zero};

use super::{error::FormulaError, types::FormulaOperator};

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaValue {
    Error(FormulaError),
    Bool(bool),
    Number(Rational64),
    String(String),
    DateTime(u64),
    Duration(i64),
    Array(Vec<FormulaValue>),
}

impl FormulaValue {
    pub fn add(self, _rhs: FormulaValue) -> FormulaValue {
        match (self.clone(), _rhs.clone()) {
            (FormulaValue::Number(a), FormulaValue::Number(b)) => FormulaValue::Number(a + b),
            (FormulaValue::String(a), FormulaValue::String(b)) => FormulaValue::String(a + &b),

            (FormulaValue::Number(a), FormulaValue::String(b)) => {
                FormulaValue::String(a.to_string() + &b)
            }
            (FormulaValue::String(a), FormulaValue::Number(b)) => {
                FormulaValue::String(a + &b.to_string())
            }

            (FormulaValue::DateTime(a), FormulaValue::Duration(b)) => {
                FormulaValue::DateTime(a + b.to_u64().unwrap())
            }
            (FormulaValue::Duration(a), FormulaValue::DateTime(b)) => {
                FormulaValue::DateTime(a.to_u64().unwrap() + b)
            }
            (FormulaValue::Duration(a), FormulaValue::Duration(b)) => FormulaValue::Duration(a + b),

            (FormulaValue::Array(a), FormulaValue::Array(b)) => {
                let mut c = a;
                c.extend(b);
                FormulaValue::Array(c)
            }
            _ => FormulaValue::Error(FormulaError::operator_mismatch_error(
                FormulaOperator::Add,
                self.into(),
                Some(_rhs.into()),
            )),
        }
    }

    pub fn sub(self, _rhs: FormulaValue) -> FormulaValue {
        match (&self, &_rhs) {
            (FormulaValue::Number(a), FormulaValue::Number(b)) => FormulaValue::Number(a - b),

            (FormulaValue::DateTime(a), FormulaValue::Duration(b)) => {
                FormulaValue::DateTime(a - b.to_u64().unwrap())
            }
            (FormulaValue::Duration(a), FormulaValue::Duration(b)) => FormulaValue::Duration(a - b),
            _ => FormulaValue::Error(FormulaError::operator_mismatch_error(
                FormulaOperator::Sub,
                self.into(),
                Some(_rhs.into()),
            )),
        }
    }

    pub fn mul(self, _rhs: FormulaValue) -> FormulaValue {
        match (&self, &_rhs) {
            (FormulaValue::Number(a), FormulaValue::Number(b)) => FormulaValue::Number(a * b),
            (FormulaValue::Duration(a), FormulaValue::Number(b)) => FormulaValue::Duration(
                (a.to_f64().unwrap() * b.to_f64().unwrap())
                    .floor()
                    .to_i64()
                    .unwrap(),
            ),
            _ => FormulaValue::Error(FormulaError::operator_mismatch_error(
                FormulaOperator::Mul,
                self.into(),
                Some(_rhs.into()),
            )),
        }
    }

    pub fn div(self, _rhs: FormulaValue) -> FormulaValue {
        match (&self, &_rhs) {
            (FormulaValue::Number(a), FormulaValue::Number(b)) => {
                if b == &Rational64::zero() {
                    FormulaValue::Error(FormulaError::divide_by_zero())
                } else {
                    FormulaValue::Number(a / b)
                }
            }

            (FormulaValue::Duration(a), FormulaValue::Number(b)) => {
                if b == &Rational64::zero() {
                    FormulaValue::Error(FormulaError::divide_by_zero())
                } else {
                    FormulaValue::Duration(
                        (a.to_f64().unwrap() / b.to_f64().unwrap())
                            .floor()
                            .to_i64()
                            .unwrap(),
                    )
                }
            }

            _ => FormulaValue::Error(FormulaError::operator_mismatch_error(
                FormulaOperator::Div,
                self.into(),
                Some(_rhs.into()),
            )),
        }
    }

    pub fn factorial(self) -> FormulaValue {
        match self {
            FormulaValue::Number(a) => {
                if a.is_integer() {
                    if a.lt(&Rational64::zero()) {
                        return FormulaValue::Error(FormulaError::factorial_not_negative());
                    }

                    let mut result = Rational64::from_integer(1);
                    for i in 1..a.to_i64().unwrap() + 1 {
                        result *= Rational64::from_integer(i);
                    }
                    FormulaValue::Number(result)
                } else {
                    FormulaValue::Error(FormulaError::factorial_not_integer())
                }
            }
            _ => FormulaValue::Error(FormulaError::operator_mismatch_error(
                FormulaOperator::Factorial,
                self.into(),
                None,
            )),
        }
    }

    pub fn pow(self, _rhs: FormulaValue) -> FormulaValue {
        match (&self, &_rhs) {
            (FormulaValue::Number(a), FormulaValue::Number(b)) => {
                if !b.is_integer() {
                    return FormulaValue::Error(FormulaError::pow_not_rational());
                }
                match b.to_i32() {
                    Some(power) => FormulaValue::Number(a.pow(power)),
                    None => FormulaValue::Error(FormulaError::number_conversion_error()),
                }
            }
            _ => FormulaValue::Error(FormulaError::operator_mismatch_error(
                FormulaOperator::Pow,
                self.into(),
                Some(_rhs.into()),
            )),
        }
    }

    pub fn rem(self, _rhs: FormulaValue) -> FormulaValue {
        match (&self, &_rhs) {
            (FormulaValue::Number(a), FormulaValue::Number(b)) => FormulaValue::Number(a % b),
            _ => FormulaValue::Error(FormulaError::operator_mismatch_error(
                FormulaOperator::Rem,
                self.into(),
                Some(_rhs.into()),
            )),
        }
    }
}

impl Display for FormulaValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FormulaValue::Error(err) => write!(f, "Error: {:?}", err),
            FormulaValue::Bool(b) => write!(f, "{}", b),
            FormulaValue::Number(n) => {
                if n.is_integer() {
                    write!(f, "{}", n.to_integer())
                } else if let Some(n) = n.to_f64() {
                    write!(f, "{}", n)
                } else {
                    write!(f, "Error")
                }
            }
            FormulaValue::String(s) => write!(f, "{}", s),
            FormulaValue::Array(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    write!(f, "{}", v)?;
                    if i != a.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            FormulaValue::DateTime(_) => write!(f, "{:?}", self),
            FormulaValue::Duration(_) => write!(f, "{:?}", self),
        }
    }
}
