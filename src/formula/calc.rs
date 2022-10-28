extern crate pest;

use alloc::{
    string::{String, ToString},
    vec,
};
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};

#[derive(Parser)]
#[grammar = "formula/calc.pest"]
pub struct Calculator;

use serde_json::Value;
use Assoc::*;
use Rule::*;

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
                return Some(ExpValue::Number(val.as_f64()?));
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
    let operators = PrecClimber::new(vec![
        Operator::new(add, Left) | Operator::new(subtract, Left),
        Operator::new(multiply, Left) | Operator::new(divide, Left),
        Operator::new(modulus, Left),
        Operator::new(power, Right),
    ]);

    operators.climb(
        expression,
        |pair| match pair.as_rule() {
            Rule::num => ExpValue::Number(pair.as_str().trim().parse::<f64>().unwrap()),
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
            _ => ExpValue::Error,
        },
        |lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs.add(rhs),
            Rule::subtract => lhs.sub(rhs),
            Rule::multiply => lhs.mul(rhs),
            Rule::divide => lhs.div(rhs),
            Rule::power => lhs.powf(rhs),
            Rule::modulus => (lhs.rem(rhs)),

            _ => ExpValue::Error,
        },
    )
}
