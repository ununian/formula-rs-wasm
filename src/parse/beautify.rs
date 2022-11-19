use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use super::ast::*;

pub trait Beautify {
    fn beautify(&self, level: usize) -> String;
}

fn indent(level: usize, str: String) -> String {
    let mut indent = String::new();
    for _ in 0..level {
        indent.push_str("  ");
    }
    indent.push_str(str.as_str());
    indent
}

impl Beautify for FormulaBody {
    fn beautify(&self, level: usize) -> String {
        format!(
            "FormulaBody\n{}",
            self.body
                .iter()
                .map(|expr| expr.1.beautify(level + 1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Beautify for ExpressionStatement {
    fn beautify(&self, level: usize) -> String {
        indent(
            level,
            format!(
                "ExpressionStatement\n{}",
                self.expression.1.beautify(level + 1)
            ),
        )
    }
}
impl Beautify for UnaryExpression {
    fn beautify(&self, level: usize) -> String {
        indent(
            level,
            format!(
                "UnaryExpression \n{}operator {}\n{}argument\n{}",
                indent(level + 1, "".to_string()),
                self.operator.1,
                indent(level + 1, "".to_string()),
                self.argument.1.beautify(level + 2)
            ),
        )
    }
}
impl Beautify for BinaryExpression {
    fn beautify(&self, level: usize) -> String {
        indent(
            level,
            format!(
                "BinaryExpression\n{}left\n{}\n{}operator {}\n{}right\n{}",
                indent(level + 1, "".to_string()),
                self.left.1.beautify(level + 2),
                indent(level + 1, "".to_string()),
                self.operator.1,
                indent(level + 1, "".to_string()),
                self.right.1.beautify(level + 2)
            ),
        )
    }
}
impl Beautify for CallExpression {
    fn beautify(&self, level: usize) -> String {
        indent(
            level,
            format!(
                "CallExpression\n{}callee\n{}\n{}arguments\n{}",
                indent(level + 1, "".to_string()),
                self.callee.1.beautify(level + 2),
                indent(level + 1, "".to_string()),
                match self.arguments.len() {
                    0 => indent(level + 2, "(EMPTY)".to_string()),
                    _ => self
                        .arguments
                        .iter()
                        .map(|expr| expr.1.beautify(level + 2))
                        .collect::<Vec<String>>()
                        .join("\n"),
                }
            ),
        )
    }
}
impl Beautify for PropertyAccessExpression {
    fn beautify(&self, level: usize) -> String {
        indent(
            level,
            format!(
                "PropertyAccessExpression\n{}object\n{}\n{}property\n{}",
                indent(level + 1, "".to_string()),
                self.object.1.beautify(level + 2),
                indent(level + 1, "".to_string()),
                self.property.1.beautify(level + 2)
            ),
        )
    }
}
impl Beautify for StringLiteral {
    fn beautify(&self, level: usize) -> String {
        indent(level, format!("StringLiteral ('{}')", self.value))
    }
}

impl Beautify for NumberLiteral {
    fn beautify(&self, level: usize) -> String {
        indent(level, format!("NumberLiteral ({})", self.value))
    }
}
impl Beautify for Identifier {
    fn beautify(&self, level: usize) -> String {
        indent(level, format!("Identifier {}", self.name))
    }
}

impl Beautify for ExpressionKind {
    fn beautify(&self, level: usize) -> String {
        match self {
            ExpressionKind::UnaryExpressionKind(_, unary_expression) => {
                unary_expression.beautify(level)
            }
            ExpressionKind::BinaryExpressionKind(_, binary_expression) => {
                binary_expression.beautify(level)
            }
            ExpressionKind::CallExpressionKind(_, call_expression) => {
                call_expression.beautify(level)
            }
            ExpressionKind::PropertyAccessExpressionKind(_, property_access_expression) => {
                property_access_expression.beautify(level)
            }
            ExpressionKind::StringLiteralKind(_, string_literal) => string_literal.beautify(level),
            ExpressionKind::NumberLiteralKind(_, number_literal) => number_literal.beautify(level),
            ExpressionKind::IdentifierKind(_, identifier) => identifier.beautify(level),
        }
    }
}
