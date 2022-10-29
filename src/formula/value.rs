use core::str::FromStr;

use alloc::{string::String, vec::Vec};
use num::{FromPrimitive, Rational64, ToPrimitive};

#[derive(Clone, Debug, PartialEq)]
pub enum ExpValue {
    Error,
    Bool(bool),
    Number(Rational64),
    String(String),
    Array(Vec<ExpValue>),
}

impl ExpValue {
    pub fn to_number(string: &str) -> ExpValue {
        match string.trim().parse::<f64>() {
            Ok(num) => match Rational64::from_f64(num) {
                Some(num) => ExpValue::Number(num),
                _ => ExpValue::Error,
            },
            _ => ExpValue::Error,
        }
    }

    pub fn add(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a + b);
            }
        }
        return ExpValue::Error;
    }

    pub fn sub(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a - b);
            }
        }
        return ExpValue::Error;
    }

    pub fn mul(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a * b);
            }
        }
        return ExpValue::Error;
    }

    pub fn div(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a / b);
            }
        }
        return ExpValue::Error;
    }

    pub fn factorial(self) -> ExpValue {
        if let ExpValue::Number(a) = self {
            let mut result = 1;
            for i in 1..(a.to_i64().unwrap_or(0) + 1) as i64 {
                result *= i;
            }
            return ExpValue::Number(Rational64::from_integer(result));
        }
        return ExpValue::Error;
    }

    pub fn powf(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                match b.to_i32() {
                    Some(b) => return ExpValue::Number(a.pow(b)),
                    None => return ExpValue::Error,
                }
            }
        }
        return ExpValue::Error;
    }

    pub fn rem(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a % b);
            }
        }
        return ExpValue::Error;
    }
}
