use std::str::FromStr;

use pest::iterators::Pair;
use serde_json::Value;

use crate::formula::{calc::Rule, ExpValue, Identifier};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parts: Vec<FunctionPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompareOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionPart {
    Identifier(String),
    IdentifierWithField(String, String),
    CompareExpression(String, CompareOperator, String),
}

impl FunctionPart {
    pub fn from(expression: Pair<Rule>) -> Vec<FunctionPart> {
        if let Rule::function_parameter = expression.as_rule() {
            return expression
                .into_inner()
                .map(FunctionPart::from_single)
                .filter(|p| p.is_some())
                .map(|p| p.unwrap())
                .collect::<Vec<FunctionPart>>();
        }
        vec![]
    }

    fn from_single(expression: Pair<Rule>) -> Option<FunctionPart> {
        if Rule::function_parameter_item == expression.as_rule() {
            let inner = expression.into_inner().collect::<Vec<Pair<Rule>>>();
            if inner.len() == 1 {
                let str = inner[0].as_str().trim();
                return str
                    .split(".")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .as_slice()
                    .split_first()
                    .map(|(first, rest)| {
                        if rest.is_empty() {
                            FunctionPart::Identifier(first.to_string())
                        } else {
                            FunctionPart::IdentifierWithField(
                                first.to_string(),
                                rest[0].to_string(),
                            )
                        }
                    });
            } else if inner.len() == 2 {
                let mut compare_expression = inner[1].clone().into_inner();

                let compare_operator = match compare_expression.next().unwrap().as_rule() {
                    Rule::compare_eq => CompareOperator::Equal,
                    _ => return None,
                };

                return Some(FunctionPart::CompareExpression(
                    inner[0].as_str().to_string(),
                    compare_operator,
                    compare_expression.next().unwrap().as_str().to_string(),
                ));
            }

            for pair in inner {
                match pair.as_rule() {
                    Rule::function_parameter_ident => {
                        return Some(FunctionPart::Identifier(pair.as_str().to_string()));
                    }
                    Rule::function_parameter_compare => {
                        let mut compare_expression = pair.into_inner();
                        let left = compare_expression.next().unwrap().as_str().to_string();
                        let operator = match compare_expression.next().unwrap().as_rule() {
                            Rule::compare_eq => CompareOperator::Equal,
                            // Rule::function_parameter_compare_not_equal => CompareOperator::NotEqual,
                            // Rule::function_parameter_compare_greater_than => {
                            //     CompareOperator::GreaterThan
                            // }
                            // Rule::function_parameter_compare_greater_than_or_equal => {
                            //     CompareOperator::GreaterThanOrEqual
                            // }
                            // Rule::function_parameter_compare_less_than => CompareOperator::LessThan,
                            // Rule::function_parameter_compare_less_than_or_equal => {
                            //     CompareOperator::LessThanOrEqual
                            // }
                            _ => return None,
                        };
                        let right = compare_expression.next().unwrap().as_str().to_string();
                        return Some(FunctionPart::CompareExpression(left, operator, right));
                    }
                    _ => return None,
                }
            }
        }
        None
    }

    pub fn get_dependencies(&self) -> Vec<Identifier> {
        match self {
            FunctionPart::Identifier(ident) => vec![Identifier {
                name: ident.clone(),
            }],
            FunctionPart::IdentifierWithField(ident, _) => vec![Identifier {
                name: ident.clone(),
            }],
            _ => vec![],
        }
    }
}

impl Function {
    pub fn from(expression: Pair<Rule>) -> Option<Function> {
        if let Rule::function = expression.as_rule() {
            let mut fun_name: Option<String> = None;
            let mut parts: Option<Vec<FunctionPart>> = None;

            for pair in expression.into_inner() {
                match pair.as_rule() {
                    Rule::function_name => {
                        fun_name = Some(pair.as_str().to_string());
                    }
                    Rule::function_parameter => {
                        parts = Some(FunctionPart::from(pair));
                    }
                    _ => continue,
                }
            }
            if let Some(name) = fun_name {
                return Some(Function {
                    name,
                    parts: parts.unwrap_or(vec![]),
                });
            }
            None
        } else {
            None
        }
    }

    pub fn get_dependencies(&self) -> Vec<Identifier> {
        self.parts
            .iter()
            .flat_map(|p| p.get_dependencies())
            .collect()
    }

    pub fn run(&self, table: &Value) -> Option<ExpValue> {
        let mut field = None;
        let mut data = match self.parts.get(0) {
            Some(FunctionPart::Identifier(ident)) => table.get(ident)?.as_array().cloned(),
            Some(FunctionPart::IdentifierWithField(ident, f)) => {
                field = Some(f);
                table.get(ident)?.as_array().cloned()
            }
            _ => return None,
        };

        data = match self.parts.get(1) {
            Some(FunctionPart::CompareExpression(left, operator, right)) => match data {
                Some(arr) => Some(
                    arr.into_iter()
                        .filter(|p| {
                            let left_value = p.get(left).unwrap().clone();
                            let right_value = Value::from_str(right);

                            if let Ok(right_value) = right_value {
                                return match operator {
                                    CompareOperator::Equal => left_value == right_value,
                                    _ => false,
                                    // CompareOperator::NotEqual => todo!(),
                                    // CompareOperator::GreaterThan => todo!(),
                                    // CompareOperator::GreaterThanOrEqual => todo!(),
                                    // CompareOperator::LessThan => todo!(),
                                    // CompareOperator::LessThanOrEqual => todo!(),
                                };
                            }
                            false
                        })
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            },
            _ => data,
        };

        data = match field {
            Some(f) => match data {
                Some(arr) => Some(
                    arr.into_iter()
                        .map(|p| p.get(f).unwrap().clone())
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            },
            _ => data,
        };

        match self.name.to_lowercase().as_str() {
            "sum" => match data {
                Some(data) => {
                    let mut sum = 0.0;
                    data.iter().for_each(|val| {
                        if val.is_number() {
                            sum += val.as_f64().unwrap_or(0.0);
                        }
                    });
                    Some(ExpValue::Number(sum))
                }
                None => None,
            },

            "count" => match data {
                Some(data) => Some(ExpValue::Number(data.len() as f64)),
                None => None,
            },
            _ => None,
        }
    }
}
