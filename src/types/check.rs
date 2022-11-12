use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;

use super::{error::TypeError, types::FormulaValueType};
use crate::parse::parse::Rule;

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

pub fn type_check(paris: &Pairs<Rule>) -> Result<FormulaValueType, TypeError> {
    TYPE_PRATT_PARSER
        .map_primary(|pair| match pair.as_rule() {
            Rule::num => Ok(FormulaValueType::Number),
            Rule::string => Ok(FormulaValueType::String),
            Rule::operation_expr => {
                let inner = pair.into_inner();
                type_check(&inner)
            }

            rule => unreachable!(
                "Expr::parse expected atom, found {:?}, value {:?}",
                rule,
                pair.as_str()
            ),
        })
        .map_infix(|lhs, op, rhs| {
            // if lhs == Ok(FormulaValueType::Error) || rhs == Ok(FormulaValueType::Error) {
            //     return Ok(FormulaValueType::Error);
            // }

            match op.as_rule() {
                Rule::add => lhs?.add(rhs?),
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            }
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::EOI => lhs,
            Rule::fac => lhs,
            _ => unreachable!(),
        })
        .parse(paris.clone())
}
