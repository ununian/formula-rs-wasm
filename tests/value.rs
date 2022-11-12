#[cfg(test)]
mod formula_value {
    use formula_rs_wasm::{
        execute::{error::ExecuteError, value::FormulaValue},
        types::{operator::FormulaOperator, types::FormulaValueType},
    };
    use num::{FromPrimitive, Rational64};

    #[test]
    fn add_success() {
        assert_eq!(
            FormulaValue::Number(1.into()).add(FormulaValue::Number(1.into())),
            FormulaValue::Number(2.into())
        );
        assert_eq!(
            FormulaValue::String("a".into()).add(FormulaValue::String("b".into())),
            FormulaValue::String("ab".into())
        );
        assert_eq!(
            FormulaValue::String("a".into()).add(FormulaValue::Number(1.into())),
            FormulaValue::String("a1".into())
        );
        assert_eq!(
            FormulaValue::Number(1.into()).add(FormulaValue::String("b".into())),
            FormulaValue::String("1b".into())
        );

        assert_eq!(
            FormulaValue::DateTime(2).add(FormulaValue::Duration(1)),
            FormulaValue::DateTime(3)
        );

        assert_eq!(
            FormulaValue::Duration(5).add(FormulaValue::DateTime(1)),
            FormulaValue::DateTime(6)
        );

        assert_eq!(
            FormulaValue::Duration(7).add(FormulaValue::Duration(1)),
            FormulaValue::Duration(8)
        );
    }

    #[test]
    fn add_error() {
        assert_eq!(
            FormulaValue::Number(1.into()).add(FormulaValue::Bool(true)),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Add,
                FormulaValueType::Number,
                Some(FormulaValueType::Bool)
            ))
        );

        assert_eq!(
            FormulaValue::Array(vec![FormulaValue::Number(1.into())])
                .add(FormulaValue::Array(vec![FormulaValue::Number(2.into())])),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Add,
                FormulaValueType::Array,
                Some(FormulaValueType::Array)
            ))
        );

        assert_eq!(
            FormulaValue::Bool(true).add(FormulaValue::Number(1.into())),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Add,
                FormulaValueType::Bool,
                Some(FormulaValueType::Number)
            ))
        );

        assert_eq!(
            FormulaValue::String("a".into()).add(FormulaValue::Bool(true)),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Add,
                FormulaValueType::String,
                Some(FormulaValueType::Bool)
            ))
        );

        assert_eq!(
            FormulaValue::DateTime(1).add(FormulaValue::Bool(true)),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Add,
                FormulaValueType::DateTime,
                Some(FormulaValueType::Bool)
            ))
        );

        assert_eq!(
            FormulaValue::Duration(1).add(FormulaValue::Bool(true)),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Add,
                FormulaValueType::Duration,
                Some(FormulaValueType::Bool)
            ))
        );

        assert_eq!(
            FormulaValue::DateTime(1).add(FormulaValue::DateTime(1)),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Add,
                FormulaValueType::DateTime,
                Some(FormulaValueType::DateTime)
            ))
        );
    }

    #[test]
    fn sub_success() {
        assert_eq!(
            FormulaValue::Number(1.into()).sub(FormulaValue::Number(1.into())),
            FormulaValue::Number(0.into())
        );

        assert_eq!(
            FormulaValue::DateTime(2).sub(FormulaValue::Duration(1)),
            FormulaValue::DateTime(1)
        );

        assert_eq!(
            FormulaValue::Duration(7).sub(FormulaValue::Duration(1)),
            FormulaValue::Duration(6)
        );
    }

    #[test]
    fn sub_error() {
        assert_eq!(
            FormulaValue::Number(1.into()).sub(FormulaValue::Bool(true)),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Sub,
                FormulaValueType::Number,
                Some(FormulaValueType::Bool)
            ))
        );

        assert_eq!(
            FormulaValue::String("".to_string()).sub(FormulaValue::String("".to_string())),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Sub,
                FormulaValueType::String,
                Some(FormulaValueType::String)
            ))
        );

        assert_eq!(
            FormulaValue::Bool(true).sub(FormulaValue::Bool(true)),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Sub,
                FormulaValueType::Bool,
                Some(FormulaValueType::Bool)
            ))
        );
    }

    #[test]
    fn mul_success() {
        assert_eq!(
            FormulaValue::Number(2.into()).mul(FormulaValue::Number(2.into())),
            FormulaValue::Number(4.into())
        );

        assert_eq!(
            FormulaValue::Duration(2).mul(FormulaValue::Number(Rational64::from_f64(2.1).unwrap())),
            FormulaValue::Duration(4)
        );
    }

    #[test]
    fn div_success() {
        assert_eq!(
            FormulaValue::Number(2.into()).div(FormulaValue::Number(2.into())),
            FormulaValue::Number(1.into())
        );

        assert_eq!(
            FormulaValue::Duration(2).div(FormulaValue::Number(Rational64::from_f64(2.1).unwrap())),
            FormulaValue::Duration(0)
        );
    }

    #[test]
    fn div_error() {
        assert_eq!(
            FormulaValue::Number(2.into())
                .div(FormulaValue::Number(Rational64::from_f64(0.0).unwrap())),
            FormulaValue::Error(ExecuteError::divide_by_zero())
        );

        assert_eq!(
            FormulaValue::Duration(2.into())
                .div(FormulaValue::Number(Rational64::from_f64(0.0).unwrap())),
            FormulaValue::Error(ExecuteError::divide_by_zero())
        );

        assert_eq!(
            FormulaValue::Number(1.into()).div(FormulaValue::Bool(true)),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Div,
                FormulaValueType::Number,
                Some(FormulaValueType::Bool)
            ))
        );

        assert_eq!(
            FormulaValue::String("".to_string()).div(FormulaValue::String("".to_string())),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Div,
                FormulaValueType::String,
                Some(FormulaValueType::String)
            ))
        );

        assert_eq!(
            FormulaValue::Bool(true).div(FormulaValue::Bool(true)),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Div,
                FormulaValueType::Bool,
                Some(FormulaValueType::Bool)
            ))
        );
    }

    #[test]
    fn pow_success() {
        assert_eq!(
            FormulaValue::Number(2.into()).pow(FormulaValue::Number(2.into())),
            FormulaValue::Number(4.into())
        );

        assert_eq!(
            FormulaValue::Number(2.into())
                .pow(FormulaValue::Number(Rational64::from_f64(0.0).unwrap())),
            FormulaValue::Number(1.into())
        );

        assert_eq!(
            FormulaValue::Number(2.into())
                .pow(FormulaValue::Number(Rational64::from_f64(1.0).unwrap())),
            FormulaValue::Number(2.into())
        );
    }

    #[test]
    fn pow_error() {
        assert_eq!(
            FormulaValue::Number(2.into())
                .pow(FormulaValue::Number(Rational64::from_f64(1.1).unwrap())),
            FormulaValue::Error(ExecuteError::pow_not_rational())
        );
    }

    #[test]
    fn factorial_success() {
        assert_eq!(
            FormulaValue::Number(0.into()).factorial(),
            FormulaValue::Number(1.into())
        );

        assert_eq!(
            FormulaValue::Number(2.into()).factorial(),
            FormulaValue::Number(2.into())
        );

        assert_eq!(
            FormulaValue::Number(3.into()).factorial(),
            FormulaValue::Number(6.into())
        );

        assert_eq!(
            FormulaValue::Number(4.into()).factorial(),
            FormulaValue::Number(24.into())
        );
    }

    #[test]
    fn factorial_error() {
        assert_eq!(
            FormulaValue::DateTime(1).factorial(),
            FormulaValue::Error(ExecuteError::operator_mismatch(
                FormulaOperator::Factorial,
                FormulaValueType::DateTime,
                None
            ))
        );

        assert_eq!(
            FormulaValue::Number(Rational64::from_f64(1.1).unwrap()).factorial(),
            FormulaValue::Error(ExecuteError::factorial_not_integer())
        );

        assert_eq!(
            FormulaValue::Number((-1).into()).factorial(),
            FormulaValue::Error(ExecuteError::factorial_not_negative())
        );
    }

    #[test]
    fn rem_success() {
        assert_eq!(
            FormulaValue::Number(2.into()).rem(FormulaValue::Number(2.into())),
            FormulaValue::Number(0.into())
        );

        assert_eq!(
            FormulaValue::Number(2.into()).rem(FormulaValue::Number(3.into())),
            FormulaValue::Number(2.into())
        );

        assert_eq!(
            FormulaValue::Number(2.into()).rem(FormulaValue::Number(4.into())),
            FormulaValue::Number(2.into())
        );
    }
}
