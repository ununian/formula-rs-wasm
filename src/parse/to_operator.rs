use core::cmp::Ordering;

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use super::ast::{
    BinaryExpression, CallExpression, ExpressionKind, ExpressionStatement, FormulaBody, Identifier,
    NumberLiteral, PropertyAccessExpression, StringLiteral, UnaryExpression,
};
use crate::share::operator::OperatorCode;

pub trait ToOperator {
    fn to_operator(&self) -> Vec<OperatorCode>;
}

impl ToOperator for FormulaBody {
    fn to_operator(&self) -> Vec<OperatorCode> {
        // for expr in self.body.iter() {
        //     result = [result, expr.1.walk()].concat();
        // }

        self.body
            .iter()
            .flat_map(|expr| expr.1.to_operator())
            .collect::<Vec<_>>()
    }
}

impl ToOperator for ExpressionStatement {
    fn to_operator(&self) -> Vec<OperatorCode> {
        self.expression.1.to_operator()
    }
}

impl ToOperator for UnaryExpression {
    fn to_operator(&self) -> Vec<OperatorCode> {
        let mut result = self.argument.1.to_operator();
        result.push(OperatorCode::Factorial);
        result
    }
}

impl ToOperator for BinaryExpression {
    fn to_operator(&self) -> Vec<OperatorCode> {
        let op = self.operator.1.as_str();
        if ["==", "=", "!=", "<>", ">=", "<=", ">", "<"].contains(&op) {
            match (&*self.left.1, &*self.right.1) {
                (
                    ExpressionKind::IdentifierKind(_, left),
                    ExpressionKind::StringLiteralKind(_, right),
                ) => {
                    vec![OperatorCode::FilterExpression(
                        String::from(left.name.clone()),
                        String::from(op),
                        String::from(right.raw.clone()),
                    )]
                }
                (
                    ExpressionKind::IdentifierKind(_, left),
                    ExpressionKind::IdentifierKind(_, right),
                ) => {
                    vec![OperatorCode::FilterExpression(
                        String::from(left.name.clone()),
                        String::from(op),
                        String::from(right.name.clone()),
                    )]
                }
                (
                    ExpressionKind::IdentifierKind(_, left),
                    ExpressionKind::NumberLiteralKind(_, right),
                ) => {
                    vec![OperatorCode::FilterExpression(
                        String::from(left.name.clone()),
                        String::from(op),
                        right.value.clone().to_string(),
                    )]
                }
                _ => {
                    vec![]
                }
            }
        } else {
            let mut result = self.left.1.to_operator();
            result.extend(self.right.1.to_operator());
            result.push(match op {
                "+" => OperatorCode::Add,
                "-" => OperatorCode::Subtract,
                "*" => OperatorCode::Multiply,
                "/" => OperatorCode::Divide,
                "%" => OperatorCode::Modulo,
                "^" => OperatorCode::Power,
                _ => unreachable!("unknown operator"),
            });
            result
        }
    }
}

impl ToOperator for NumberLiteral {
    fn to_operator(&self) -> Vec<OperatorCode> {
        vec![OperatorCode::PushNumber(self.value)]
    }
}

impl ToOperator for Identifier {
    fn to_operator(&self) -> Vec<OperatorCode> {
        vec![OperatorCode::LoadIdentifier(self.name.clone())]
    }
}

impl ToOperator for StringLiteral {
    fn to_operator(&self) -> Vec<OperatorCode> {
        vec![OperatorCode::PushString(self.value.clone())]
    }
}

impl ToOperator for CallExpression {
    fn to_operator(&self) -> Vec<OperatorCode> {
        // TODO: 为了实现奇怪的过滤方式，这里受到的影响最大，需要重构
        let mut result = self.callee.1.to_operator();

        let mut args = self
            .arguments
            .iter()
            .flat_map(|arg| arg.1.to_operator())
            .collect::<Vec<_>>();

        // Order: LoadIdentifier > FilterExpression > LoadPropertyAccess > Call
        args.sort_by(|a, b| {
            // TODO: 这个排序就固定了函数的调用方式，需要重构
            let a = match a {
                OperatorCode::LoadIdentifier(_) => 0,
                OperatorCode::FilterExpression(_, _, _) => 1,
                OperatorCode::LoadPropertyAccess(_) => 2,
                OperatorCode::Call(_) => 3,
                _ => 4,
            };
            let b = match b {
                OperatorCode::LoadIdentifier(_) => 0,
                OperatorCode::FilterExpression(_, _, _) => 1,
                OperatorCode::LoadPropertyAccess(_) => 2,
                OperatorCode::Call(_) => 3,
                _ => 4,
            };
            a.cmp(&b)
        });

        result.extend(args);

        let has_filter = result.iter().any(|arg| {
            if let OperatorCode::FilterExpression(_, _, _) = arg {
                return true;
            }
            false
        });

        if has_filter {
            result.push(OperatorCode::Call((self.arguments.len() - 1) as u8));
        } else {
            result.push(OperatorCode::Call(self.arguments.len() as u8));
        }
        result
    }
}

impl ToOperator for PropertyAccessExpression {
    fn to_operator(&self) -> Vec<OperatorCode> {
        let mut result = self.object.1.to_operator();
        result.push(OperatorCode::LoadPropertyAccess(
            self.property.1.name.clone(),
        ));
        result
    }
}

impl ToOperator for ExpressionKind {
    fn to_operator(&self) -> Vec<OperatorCode> {
        match self {
            ExpressionKind::UnaryExpressionKind(_, unary_expression) => {
                unary_expression.to_operator()
            }
            ExpressionKind::BinaryExpressionKind(_, binary_expression) => {
                binary_expression.to_operator()
            }
            ExpressionKind::CallExpressionKind(_, call_expression) => call_expression.to_operator(),
            // ExpressionKind::PropertyAccessExpressionKind(_, property_access_expression) => {
            //     property_access_expression.to_operator()
            // }
            ExpressionKind::StringLiteralKind(_, string_literal) => string_literal.to_operator(),
            ExpressionKind::NumberLiteralKind(_, number_literal) => number_literal.to_operator(),
            ExpressionKind::IdentifierKind(_, identifier) => identifier.to_operator(),
            // ExpressionKind::TypeDefineKind(_, type_define) => type_define.to_operator(),
            ExpressionKind::PropertyAccessExpressionKind(_, dot) => dot.to_operator(),
            _ => todo!("not implemented"),
        }
    }
}
