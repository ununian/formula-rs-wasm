use alloc::string::String;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;

use super::error::FormulaError;
use super::formula::{Formula, Rule};
use super::types::FormulaValueType;

lazy_static::lazy_static! {
    static ref TYPE_PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(modulus, Left))
            .op(Op::infix(power, Right))
            .op(Op::postfix(Rule::fac))
            .op(Op::postfix(EOI))
    };
}

pub fn type_check(paris: &Pairs<Rule>) -> Result<FormulaValueType, FormulaError> {
    let mut error = None;
    let type_result = TYPE_PRATT_PARSER
        .map_primary(|pair| match pair.as_rule() {
            Rule::num => FormulaValueType::Number,
            Rule::string => FormulaValueType::String,
            Rule::operation_expr => {
                let inner = pair.into_inner();
                let result = type_check(&inner);
               
                match result {
                    Ok(t) => t,
                    Err(_) => FormulaValueType::Error,
                }

                // let mut inner = pair.into_inner();
                // let expr = inner.next().unwrap();
                // let result = type_check(&expr.into_inner());
                // if result.is_err() {
                //     return FormulaValueType::Error
                // }
                // result.unwrap()
            }

            rule => unreachable!("Expr::parse expected atom, found {:?}, value {:?}", rule, pair.as_str()),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => match (lhs, rhs) {
                (FormulaValueType::Number, FormulaValueType::Number) => FormulaValueType::Number,
                _ => {
                    error = Some(FormulaError::type_mismatch_error());
                    FormulaValueType::Error
                }
            },
            rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::EOI => lhs,
            Rule::fac => lhs,
            _ => unreachable!(),
        })
        .parse(paris.clone());

    match error {
        Some(e) => Err(e),
        None => Ok(type_result),
    }
}
