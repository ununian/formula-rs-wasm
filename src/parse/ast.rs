use core::str::FromStr;

use super::{
    parse::Rule,
    type_ast::{type_def_to_ast, TypeDefine},
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use num::{FromPrimitive, Rational64};
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
pub enum AstKind {
    FormulaBodyKind(FormulaBody),                 // 公式
    ExpressionStatementKind(ExpressionStatement), // 表达式语句
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind {
    UnaryExpressionKind(Range, UnaryExpression), // 一元表达式
    BinaryExpressionKind(Range, BinaryExpression), // 二元表达式
    CallExpressionKind(Range, CallExpression),   // 函数调用表达式
    PropertyAccessExpressionKind(Range, PropertyAccessExpression), // 属性访问表达式，即 Dot 运算符

    StringLiteralKind(Range, StringLiteral), // 字符串字面量
    NumberLiteralKind(Range, NumberLiteral), // 数字字面量
    IdentifierKind(Range, Identifier),       // 标识符

    TypeDefineKind(Range, TypeDefine), // 类型定义
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionAstItem(pub Range, pub ExpressionKind);

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaBody {
    pub body: Vec<(Range, ExpressionStatement)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: ExpressionAstItem,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpression {
    pub prefix: bool,

    pub operator: (Range, String),
    pub argument: (Range, Box<ExpressionKind>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression {
    pub left: (Range, Box<ExpressionKind>),
    pub operator: (Range, String),
    pub right: (Range, Box<ExpressionKind>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpression {
    pub callee: (Range, Box<ExpressionKind>),
    pub arguments: Vec<(Range, Box<ExpressionKind>)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyAccessExpression {
    pub object: (Range, Box<ExpressionKind>),
    pub property: (Range, Identifier),
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
            // 比较运算
            .op(
                Op::infix(compare_eq, Left)     |
                Op::infix(compare_ne, Left)     |
                Op::infix(compare_ge, Left)     |
                Op::infix(compare_le, Left)     |
                Op::infix(compare_lt , Left)    |
                Op::infix(compare_gt, Left)
            )

            // 四则运算
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(modulus, Left))
            .op(Op::infix(power, Right))

            .op(Op::postfix(Rule::fac))
            .op(Op::postfix(Rule::dot))
            .op(Op::postfix(EOI))
    };
}

pub fn variable_to_ast(pair: Pair<Rule>) -> ExpressionAstItem {
    fn dot_to_property_access(pairs: &[Pair<Rule>]) -> ExpressionAstItem {
        if pairs.len() == 0 {
            unreachable!("dot_to_property_access: pairs.len() == 0");
        } else {
            let first = pairs[0].clone();
            if pairs.len() == 1 {
                if first.as_rule() == Rule::identifier {
                    let range = Range::from(pairs[0].clone());
                    let identifier = Identifier {
                        name: pairs[0].as_str().to_string(),
                    };
                    return ExpressionAstItem(
                        range.clone(),
                        ExpressionKind::IdentifierKind(range, identifier),
                    );
                } else {
                    unreachable!("dot_to_property_access: need Rule::identifier");
                }
            } else if pairs[0].as_rule() == Rule::dot {
                let range = Range(
                    first.as_span().start(),
                    pairs[pairs.len() - 1].as_span().end(),
                );
                let object = dot_to_property_access(&pairs[1..pairs.len()]);

                let property_pair = first.into_inner().next().unwrap();
                let property = match property_pair.as_rule() {
                    Rule::identifier => {
                        let range = Range::from(pairs[pairs.len() - 1].clone());
                        let identifier = Identifier {
                            name: property_pair.as_str().to_string(),
                        };
                        (range,identifier)
                    }
                    _ => unreachable!("dot_to_property_access: pairs[0].into_inner().next().unwrap().as_rule() != Rule::identifier"),
                };
                return ExpressionAstItem(
                    range.clone(),
                    ExpressionKind::PropertyAccessExpressionKind(
                        range,
                        PropertyAccessExpression {
                            object: (object.0, Box::new(object.1)),
                            property,
                        },
                    ),
                );
            } else {
                unreachable!("dot_to_property_access: pairs[0].as_rule() != Rule::dot");
            }
        }
    }

    let range = Range::from(pair.clone());
    let inner = pair.into_inner();

    let count = inner.clone().count();

    if count == 1 {
        let pair = inner.clone().next().unwrap();
        match pair.as_rule() {
            Rule::identifier => {
                let identifier = Identifier {
                    name: pair.as_str().to_string(),
                };
                ExpressionAstItem(
                    range.clone(),
                    ExpressionKind::IdentifierKind(range, identifier),
                )
            }
            _ => unreachable!(),
        }
    } else if count > 1 {
        let mut arr = inner.collect::<Vec<_>>();
        arr.reverse();
        dot_to_property_access(&arr)
    } else {
        unreachable!()
    }
}

pub fn literal_to_ast(pair: Pair<Rule>) -> ExpressionAstItem {
    let range = Range::from(pair.clone());

    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();
    match first.as_rule() {
        Rule::num => {
            let raw = first.as_str().to_string();

            let value = match raw.contains(".") {
                true => Rational64::from_f64(raw.parse().unwrap()).unwrap(),
                false => Rational64::from_str(&raw).unwrap(),
            };

            ExpressionAstItem(
                range.clone(),
                ExpressionKind::NumberLiteralKind(range, NumberLiteral { value, raw }),
            )
        }
        Rule::string => {
            let raw = first.as_str();
            ExpressionAstItem(
                range.clone(),
                ExpressionKind::StringLiteralKind(
                    range,
                    StringLiteral {
                        value: raw[1..raw.len() - 1].to_string(),
                        raw: raw.to_string(),
                    },
                ),
            )
        }
        _ => unreachable!(),
    }
}

pub fn variable_or_expression(pair: Pair<Rule>) -> ExpressionAstItem {
    let _rule = pair.as_rule();
    let _txt = pair.as_str();
    match pair.as_rule() {
        Rule::variable => variable_to_ast(pair),
        Rule::expr => expression_to_ast(pair.into_inner()),
        Rule::operation_expr => expression_to_ast(pair.into_inner()),
        Rule::compare_expr => expression_to_ast(pair.into_inner()),
        Rule::identifier => {
            let identifier = Identifier {
                name: pair.as_str().to_string(),
            };
            ExpressionAstItem(
                pair.clone().into(),
                ExpressionKind::IdentifierKind(pair.clone().into(), identifier),
            )
        }
        Rule::function_call => function_call_to_ast(pair),
        _ => unreachable!(),
    }
}

pub fn variable_or_literal_or_expression(pair: Pair<Rule>) -> ExpressionAstItem {
    match pair.as_rule() {
        Rule::literal => literal_to_ast(pair),
        _ => variable_or_expression(pair),
    }
}

fn function_call_to_ast(pair: Pair<Rule>) -> ExpressionAstItem {
    let mut pairs = pair.clone().into_inner();

    let callee = pairs.next().unwrap();

    let callee = variable_or_expression(callee);

    let _text = pairs.clone().map(|p| p.as_str()).collect::<Vec<_>>();

    let arguments = pairs
        .flat_map(|arguments| {
            arguments
                .into_inner()
                .map(|pair| variable_or_literal_or_expression(pair))
                .map(|ExpressionAstItem(range, expression)| (range, Box::new(expression)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    ExpressionAstItem(
        pair.clone().into(),
        ExpressionKind::CallExpressionKind(
            pair.clone().into(),
            CallExpression {
                callee: (callee.0, Box::new(callee.1)),
                arguments,
            },
        ),
    )
}

pub fn expression_to_ast(paris: Pairs<Rule>) -> ExpressionAstItem {
    TYPE_PRATT_PARSER
        .map_primary(|pair| match pair.as_rule() {
            Rule::literal => literal_to_ast(pair),
            Rule::function_call => function_call_to_ast(pair),
            Rule::variable => variable_to_ast(pair),
            Rule::identifier => ExpressionAstItem(
                pair.clone().into(),
                ExpressionKind::IdentifierKind(
                    pair.clone().into(),
                    Identifier {
                        name: pair.as_str().to_string(),
                    },
                ),
            ),

            Rule::operation_expr => {
                let inner = pair.into_inner();
                expression_to_ast(inner)
            }
            Rule::compare_expr => {
                let inner = pair.into_inner();
                expression_to_ast(inner)
            }
            Rule::type_def => type_def_to_ast(pair),

            rule => unreachable!(
                "Expr::parse expected atom, found {:?}, value {:?}",
                rule,
                pair.as_str()
            ),
        })
        .map_infix(|lhs, op, rhs| {
            ExpressionAstItem(
                Range(lhs.0 .0, rhs.0 .1),
                ExpressionKind::BinaryExpressionKind(
                    Range(lhs.0 .0, rhs.0 .1),
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
            Rule::fac => {
                let range = Range(lhs.0 .0, op.as_span().end());
                ExpressionAstItem(
                    range.clone(),
                    ExpressionKind::UnaryExpressionKind(
                        range,
                        UnaryExpression {
                            prefix: false,
                            operator: (op.clone().into(), op.as_str().to_string()),
                            argument: (lhs.0, Box::new(lhs.1)),
                        },
                    ),
                )
            }
            Rule::dot => {
                let inner = op.into_inner();
                let ExpressionAstItem(range, _) = expression_to_ast(inner);
                ExpressionAstItem(
                    Range(lhs.0 .0, range.1),
                    ExpressionKind::PropertyAccessExpressionKind(
                        Range(lhs.0 .0, range.1),
                        PropertyAccessExpression {
                            object: (lhs.0, Box::new(lhs.1)),
                            property: (
                                range,
                                Identifier {
                                    name: "a".to_string(),
                                },
                            ),
                        },
                    ),
                )
            }
            _ => unreachable!(),
        })
        .parse(paris)
}

pub fn to_ast(paris: Pairs<Rule>) -> (Range, FormulaBody) {
    let mut statements = Vec::new();
    for pair in paris.clone() {
        match pair.as_rule() {
            Rule::statement => {
                let inner = pair.into_inner();

                let expression = expression_to_ast(inner);
                statements.push((expression.0.clone(), ExpressionStatement { expression }));
            }
            Rule::EOI => {}
            _ => {}
        }
    }
    (
        Range(0, paris.last().unwrap().as_span().end()),
        FormulaBody { body: statements },
    )
}
