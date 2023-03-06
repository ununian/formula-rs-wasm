#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};

#[macro_use]
extern crate pest_derive;

pub mod execute;
pub mod parse;
pub mod share;
pub mod types;
mod utils;
pub mod vm;

use alloc::vec::Vec;
use parse::ast::to_ast;
use parse::dependencies::get_dependencies;
use parse::parse::Formula;
use parse::to_operator::ToOperator;
use serde_json::Value as JsonValue;
use vm::context::RuntimeContext;
use vm::runner::Runner;

use vm::value::Value;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn ping() -> String {
    // let formula = formula::calc::parse(&expr).unwrap();
    // let table: Value = serde_json::from_str(&data).unwrap();
    // return format!("{}", formula::calc::eval(formula, &table));
    String::from("Pang")
}

#[wasm_bindgen]
pub fn compile(expr: String) -> Vec<u8> {
    let formula = Formula::parse(expr.as_str()).unwrap();
    let (_, ast) = to_ast(formula.paris);
    let operators = ast.to_operator();
    let encoded: Vec<u8> = bincode::serialize(&operators).unwrap();
    encoded
}

#[wasm_bindgen]
pub fn run(expr: String, data: String, now: i64, today: i64) -> String {
    let formula = Formula::parse(expr.as_str()).unwrap();
    let (_, ast) = to_ast(formula.paris);
    let operators = ast.to_operator();

    let dependencies = get_dependencies(&operators);

    let issue: JsonValue = serde_json::from_str(&data).unwrap();

    let runner = Runner;
    let mut ctx = RuntimeContext::new();
    ctx.inject_functions();
    mock_time(&mut ctx, &issue, now, today);

    for dependency in dependencies {
        if ctx.has(&dependency) {
            continue;
        }
        let value = Value::from_json(&issue[&dependency]);
        // println!("dependency: {}", &dependency);
        ctx.set(dependency, value);
    }

    let result = runner.run(operators, &mut ctx);

    format!("{}", result.unwrap())
}

fn mock_time(ctx: &mut RuntimeContext, issue: &JsonValue, now: i64, today: i64) {
    ctx.set(
        "GET_TODAY".to_string(),
        Value::Number((today as i64).into()),
    );
    ctx.set("GET_NOW".to_string(), Value::Number((now as i64).into()));

    ctx.set(
        "GET_UPDATE_TIME".to_string(),
        Value::from_json(&issue["updateTime"]),
    );

    ctx.set(
        "GET_CREATE_TIME".to_string(),
        Value::from_json(&issue["createTime"]),
    );
}
