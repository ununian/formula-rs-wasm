use alloc::vec::Vec;

use super::ast::{ExpressionKind, ExpressionStatement, FormulaBody};

impl FormulaBody {
    pub fn walk(&self) -> Vec<ExpressionKind> {
        let mut result = Vec::new();

        for expr in self.body.iter() {
            result = [result, expr.1.walk()].concat();
        }

        result
    }
}

impl ExpressionStatement {
    pub fn walk(&self) -> Vec<ExpressionKind> {
        self.expression.1.walk()
    }
}

impl ExpressionKind {
    pub fn walk(&self) -> Vec<ExpressionKind> {
        let result = Vec::new();

        

        result
    }
}
