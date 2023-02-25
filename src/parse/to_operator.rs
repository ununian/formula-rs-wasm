use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use super::ast::{
    BinaryExpression, ExpressionKind, ExpressionStatement, FormulaBody, Identifier, NumberLiteral,
    UnaryExpression, StringLiteral,
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
        let mut result = self.left.1.to_operator();
        result.extend(self.right.1.to_operator());
        result.push(match self.operator.1.as_str() {
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

impl ToOperator for NumberLiteral {
    fn to_operator(&self) -> Vec<OperatorCode> {
        vec![OperatorCode::PushNumber(self.value)]
    }
}

impl ToOperator for Identifier {
    fn to_operator(&self) -> Vec<OperatorCode> {
        vec![OperatorCode::LoadIdentifier(self.name.as_str())]
    }
}

impl ToOperator for StringLiteral {
    fn to_operator(&self) -> Vec<OperatorCode> {
        vec![OperatorCode::PushString(self.value.as_str())]
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
            // ExpressionKind::CallExpressionKind(_, call_expression) => call_expression.to_operator(),
            // ExpressionKind::PropertyAccessExpressionKind(_, property_access_expression) => {
            //     property_access_expression.to_operator()
            // }
            ExpressionKind::StringLiteralKind(_, string_literal) => string_literal.to_operator(),
            ExpressionKind::NumberLiteralKind(_, number_literal) => number_literal.to_operator(),
            ExpressionKind::IdentifierKind(_, identifier) => identifier.to_operator(),
            // ExpressionKind::TypeDefineKind(_, type_define) => type_define.to_operator(),
            _ => vec![],
        }
    }
}
