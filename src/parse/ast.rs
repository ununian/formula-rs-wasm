use core::str::FromStr;

use super::parse::Rule;
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use num::Rational64;
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Range(usize, usize);

impl From<Pair<'_, Rule>> for Range {
    fn from(pair: Pair<Rule>) -> Self {
        Self(pair.as_span().start(), pair.as_span().end())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstType {
    FormulaBody(FormulaBody),                 // 公式
    ExpressionStatement(ExpressionStatement), // 表达式语句
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionType {
    UnaryExpression(Range, UnaryExpression),   // 一元表达式
    BinaryExpression(Range, BinaryExpression), // 二元表达式
    CallExpression(Range, CallExpression),     // 函数调用表达式
    PropertyAccessExpression(Range, PropertyAccessExpression), // 属性访问表达式，即 Dot 运算符

    StringLiteral(Range, StringLiteral), // 字符串字面量
    NumberLiteral(Range, NumberLiteral), // 数字字面量
    Identifier(Range, Identifier),       // 标识符
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaBody {
    pub body: Vec<(Range, ExpressionStatement)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: (Range, ExpressionType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpression {
    pub prefix: bool,

    pub operator: (Range, String),
    pub argument: (Range, Box<ExpressionType>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: (Range, Box<ExpressionType>),
    pub operator: (Range, String),
    pub right: (Range, Box<ExpressionType>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpression {
    pub callee: (Range, Box<ExpressionType>),
    pub arguments: Vec<(Range, Box<ExpressionType>)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyAccessExpression {
    pub object: (Range, Box<ExpressionType>),
    pub property: (Range, Box<ExpressionType>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringLiteral {
    pub value: String,
    pub raw: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberLiteral {
    pub value: Rational64,
    pub raw: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

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

pub fn to_ast(paris: Pairs<Rule>) -> (Range, ExpressionType) {
    TYPE_PRATT_PARSER
        .map_primary(|pair| match pair.as_rule() {
            Rule::num => (
                pair.clone().into(),
                ExpressionType::NumberLiteral(
                    pair.clone().into(),
                    NumberLiteral {
                        value: Rational64::from_str(pair.as_str()).unwrap(),
                        raw: pair.as_str().to_string(),
                    },
                ),
            ),
            Rule::string => {
                let len = pair.as_str().len();
                (
                    pair.clone().into(),
                    ExpressionType::StringLiteral(
                        pair.clone().into(),
                        StringLiteral {
                            raw: pair.as_str().to_string(),
                            value: pair.as_str()[1..len - 1].to_string(),
                        },
                    ),
                )
            }
            Rule::operation_expr => {
                let inner = pair.into_inner();
                to_ast(inner)
            }
            rule => unreachable!(
                "Expr::parse expected atom, found {:?}, value {:?}",
                rule,
                pair.as_str()
            ),
        })
        .map_infix(|lhs, op, rhs| {
            (
                Range(lhs.0 .0, rhs.0 .1),
                ExpressionType::BinaryExpression(
                    op.clone().into(),
                    BinaryExpression {
                        left: (lhs.0, Box::new(lhs.1)),
                        operator: (op.clone().into(), op.as_str().to_string()),
                        right: (rhs.0, Box::new(rhs.1)),
                    },
                ),
            )
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::EOI => lhs,
            Rule::fac => lhs,
            _ => unreachable!(),
        })
        .parse(paris)
}
