use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::share::operator::OperatorCode;

pub fn get_dependencies(codes: &Vec<OperatorCode>) -> Vec<String> {
    let function_name = ["SUM".to_string(), "COUNT".to_string()];

    codes
        .iter()
        .map(|code| match code {
            OperatorCode::LoadIdentifier(name) => Some(name),
            _ => None,
        })
        .filter(|name| name.is_some() && !&function_name.contains(name.unwrap()))
        .map(|func| func.unwrap().clone())
        .collect::<Vec<_>>()
}
