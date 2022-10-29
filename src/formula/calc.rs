extern crate pest;

use core::str::FromStr;

use alloc::string::{String, ToString};
use num::{FromPrimitive, Rational64, Zero};
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};

#[derive(Parser)]
#[grammar = "formula/calc.pest"]
pub struct Calculator;

use serde_json::Value;

use crate::formula::{ExpValue, Function};

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    Calculator::parse(Rule::calculation, input)
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn get_value(self, table: &Value) -> Option<ExpValue> {
        let val = table.as_object()?.get(&self.name);
        if let Some(val) = val {
            if val.is_string() {
                return Some(ExpValue::String(val.as_str()?.to_string()));
            } else if val.is_number() {
                // return Some(ExpValue::Number(
                //     Rational64::from_f64(val.as_f64().unwrap_or(default)).unwrap_or(Rational64::zero()),
                // ));
                match val.as_f64() {
                    Some(f) => {
                        return Some(ExpValue::Number(
                            Rational64::from_f64(f).unwrap_or(Rational64::zero()),
                        ))
                    }
                    None => return Some(ExpValue::Error),
                }
            }
            return Some(ExpValue::Error);
        }
        None
    }

    pub fn from(expression: Pair<Rule>) -> Option<Identifier> {
        if let Rule::ident = expression.as_rule() {
            return Some(Identifier {
                name: expression.as_str().to_string(),
            });
        }
        None
    }
}

pub fn eval(expression: Pairs<Rule>, table: &Value) -> ExpValue {
    use Assoc::*;
    use Rule::*;

    let pratt = PrattParser::new()
        .op(Op::infix(add, Left) | Op::infix(subtract, Left))
        .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
        .op(Op::infix(modulus, Left))
        .op(Op::infix(power, Right))
        .op(Op::prefix(Rule::neg))
        .op(Op::postfix(Rule::fac))
        .op(Op::postfix(EOI));

    pratt
        .map_primary(|pair| match pair.as_rule() {
            Rule::num => ExpValue::to_number(pair.as_str()),
            Rule::expr => eval(pair.into_inner(), &table),
            Rule::ident => {
                let name = pair.as_str().trim();
                let id = Identifier {
                    name: name.to_string(),
                };
                id.get_value(&table).unwrap_or(ExpValue::Error)
            }
            Rule::function => Function::from(pair)
                .unwrap()
                .run(table)
                .unwrap_or(ExpValue::Error),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs.add(rhs),
            Rule::subtract => lhs.sub(rhs),
            Rule::multiply => lhs.mul(rhs),
            Rule::divide => lhs.div(rhs),
            Rule::power => lhs.powf(rhs),
            Rule::modulus => lhs.rem(rhs),
            rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::EOI => lhs,
            Rule::fac => lhs.factorial(),
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::neg => ExpValue::Number(Rational64::zero()).sub(rhs),
            _ => unreachable!(),
        })
        .parse(expression.clone())
}
