use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use pest::iterators::{Pair, Pairs};

use crate::formula::{calc::Identifier, calc::Rule, function::Function};

#[derive(Debug, Clone)]
pub struct Expression {
    pub parts: Vec<ExpressionPart>,
}

#[derive(Debug, Clone)]
pub enum ExpressionPart {
    Identifier(Identifier),
    Function(Function),
    Operator(String),
}

impl Expression {
    pub fn from_pairs(mut expression: Pairs<Rule>) -> Option<Expression> {
        match expression.next() {
            Some(root) => Expression::from(root),
            None => None,
        }
    }

    pub fn from(expression: Pair<Rule>) -> Option<Expression> {
        let mut expression = Some(expression);

        expression = match expression {
            Some(expr) => {
                if let Rule::calculation = expr.as_rule() {
                    expr.into_inner().next()
                } else {
                    Some(expr)
                }
            }
            None => None,
        };

        expression = match expression {
            Some(expr) => {
                if let Rule::stmt = expr.as_rule() {
                    expr.into_inner().next()
                } else {
                    Some(expr)
                }
            }
            None => None,
        };

        match expression {
            Some(expr) => {
                if expr.as_rule() == Rule::expr {
                    return Some(Expression {
                        parts: expr
                            .into_inner()
                            .map(ExpressionPart::from)
                            .filter(|p| p.is_some())
                            .map(|p| p.unwrap())
                            .collect::<Vec<ExpressionPart>>(),
                    });
                }
                None
            }
            None => None,
        }
    }

    pub fn get_dependencies(&self) -> Vec<Identifier> {
        self.parts
            .iter()
            .flat_map(|p| match p {
                ExpressionPart::Identifier(i) => vec![i.clone()],
                ExpressionPart::Function(func) => func.get_dependencies(),
                _ => vec![],
            })
            .collect()
    }
}

impl ExpressionPart {
    pub fn from(expression: Pair<Rule>) -> Option<ExpressionPart> {
        match expression.as_rule() {
            Rule::function => {
                return match Function::from(expression) {
                    Some(i) => Some(ExpressionPart::Function(i)),
                    None => None,
                };
            }
            Rule::ident => {
                return match Identifier::from(expression) {
                    Some(i) => Some(ExpressionPart::Identifier(i)),
                    None => None,
                };
            }
            Rule::operation => {
                return Some(ExpressionPart::Operator(expression.as_str().to_string()));
            }
            _ => None,
        }
    }
}
