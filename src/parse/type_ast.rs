use super::ast::{ExpressionAstItem, ExpressionKind, Identifier, Range};

use super::parse::Rule;
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use pest::iterators::{Pair, Pairs};

#[derive(Clone, Debug, PartialEq)]
pub struct TypeName {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeDefine {
    pub ident: (Range, Identifier),
    pub type_item: (Range, TypeItemKind),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeItemKind {
    NamedTypeKind(NamedType),   // 具名类型
    RecordTypeKind(RecordType), // 键值对类型
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordTypeField {
    pub key: (Range, Identifier),
    pub value: (Range, TypeItemKind),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordType {
    pub fields: Vec<(Range, RecordTypeField)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedType {
    pub ident: (Range, TypeName),
    pub parameters: Vec<(Range, TypeItemKind)>,
}

fn named_type_to_ast(pair: Pair<Rule>) -> (Range, NamedType) {
    let range = Range::from(pair.clone());
    let mut inner = pair.into_inner();

    let ident = match inner.next() {
        Some(pair) => match pair.as_rule() {
            Rule::type_name => (
                pair.clone().into(),
                TypeName {
                    name: pair.as_str().to_string(),
                },
            ),
            _ => unreachable!(),
        },
        None => panic!("type_item_to_ast: inner.next() == None"),
    };

    let parameters = match inner.next() {
        Some(pair) => match pair.as_rule() {
            Rule::type_parameters => pair
                .into_inner()
                .map(|pair| type_item_to_ast(pair.into_inner()))
                .collect::<Vec<_>>(),
            _ => unreachable!(),
        },
        None => vec![],
    };

    (range, NamedType { ident, parameters })
}

fn record_item_type_to_ast(pair: Pair<Rule>) -> (Range, RecordTypeField) {
    let range = Range::from(pair.clone());
    let mut inner = pair.into_inner();

    let key = match inner.next() {
        Some(pair) => match pair.as_rule() {
            Rule::type_record_type_key => (
                pair.clone().into(),
                Identifier {
                    name: pair.as_str().to_string(),
                },
            ),
            _ => unreachable!(),
        },
        None => panic!("record_item_type_to_ast: inner.next() == None"),
    };

    let value = match inner.next() {
        Some(pair) => match pair.as_rule() {
            Rule::type_item => type_item_to_ast(pair.into_inner()),
            _ => unreachable!(),
        },
        None => panic!("record_item_type_to_ast: inner.next() == None"),
    };

    (range, RecordTypeField { key, value })

    // panic!(
    //     "record_type_to_ast: {:?}",
    //     inner.clone().map(|f| f.as_rule()).collect::<Vec<_>>()
    // );
}

fn record_type_to_ast(pair: Pair<Rule>) -> (Range, RecordType) {
    (
        Range::from(pair.clone()),
        RecordType {
            fields: pair
                .into_inner()
                .map(record_item_type_to_ast)
                .collect::<Vec<_>>(),
        },
    )
}

pub fn type_item_to_ast(mut pairs: Pairs<Rule>) -> (Range, TypeItemKind) {
    //   panic!(
    //     "type_item_to_ast: {:?}",
    //     pairs.clone().map(|f| f.as_rule()).collect::<Vec<_>>()
    // );

    match pairs.next() {
        Some(pair) => match pair.as_rule() {
            Rule::type_named_type => {
                let named_type = named_type_to_ast(pair);
                (named_type.0, TypeItemKind::NamedTypeKind(named_type.1))
            }
            Rule::type_record_type => {
                let record_type = record_type_to_ast(pair);
                (record_type.0, TypeItemKind::RecordTypeKind(record_type.1))
            }
            _ => unreachable!(),
        },
        None => unreachable!(),
    }

    // panic!(
    //     "type_item_to_ast: {:?}",
    //     inner.clone().map(|f| f.as_str()).collect::<Vec<_>>()
    // );
}

pub fn type_def_to_ast(pair: Pair<Rule>) -> ExpressionAstItem {
    let mut pairs = pair.clone().into_inner();

    match pairs.next() {
        Some(kw) => {
            if kw.as_str() != "type" {
                panic!(
                    "type_def_to_ast: keyword error, expected 'type', got '{}'",
                    kw.as_str()
                );
            }
        }
        None => panic!("type_def_to_ast: no keyword"),
    }

    let identifier = match pairs.next() {
        Some(ident) => {
            if ident.as_rule() != Rule::identifier {
                panic!(
                    "type_def_to_ast: identifier error, expected identifier, got '{}'",
                    ident.as_str()
                );
            }
            (
                ident.clone().into(),
                Identifier {
                    name: ident.as_str().to_string(),
                },
            )
        }
        None => panic!("type_def_to_ast: no identifier"),
    };

    let type_item = match pairs.next() {
        Some(item) => {
            if item.as_rule() != Rule::type_item {
                panic!(
                    "type_def_to_ast: type error, expected type, got '{}'",
                    item.as_str()
                );
            }
            type_item_to_ast(item.into_inner())
        }
        None => panic!("type_def_to_ast: no type"),
    };

    ExpressionAstItem(
        pair.clone().into(),
        ExpressionKind::TypeDefineKind(
            pair.clone().into(),
            TypeDefine {
                ident: identifier,
                type_item,
            },
        ),
    )
}
