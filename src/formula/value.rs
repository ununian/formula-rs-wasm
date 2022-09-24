use std::ops;

#[derive(Clone, Debug, PartialEq)]
pub enum ExpValue {
    Error,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<ExpValue>),
}

impl ops::Add<ExpValue> for ExpValue {
    type Output = ExpValue;
    fn add(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a + b);
            }
        }
        return ExpValue::Error;
    }
}

impl ops::Sub<ExpValue> for ExpValue {
    type Output = ExpValue;
    fn sub(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a - b);
            }
        }
        return ExpValue::Error;
    }
}

impl ops::Mul<ExpValue> for ExpValue {
    type Output = ExpValue;
    fn mul(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a * b);
            }
        }
        return ExpValue::Error;
    }
}

impl ops::Div<ExpValue> for ExpValue {
    type Output = ExpValue;
    fn div(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a / b);
            }
        }
        return ExpValue::Error;
    }
}

impl ExpValue {
    pub fn powf(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a.powf(b));
            }
        }
        return ExpValue::Error;
    }
}

impl ops::Rem<ExpValue> for ExpValue {
    type Output = ExpValue;
    fn rem(self, _rhs: ExpValue) -> ExpValue {
        if let ExpValue::Number(a) = self {
            if let ExpValue::Number(b) = _rhs {
                return ExpValue::Number(a % b);
            }
        }
        return ExpValue::Error;
    }
}
