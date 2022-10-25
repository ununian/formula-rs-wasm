use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, PartialEq)]
pub enum ExpValue {
    Error,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<ExpValue>),
}

impl ExpValue {
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

    pub fn powf(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a.powf(b));
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
