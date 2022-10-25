#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::format;

#[macro_use]
extern crate pest_derive;

pub mod formula;
mod utils;

use serde_json::Value;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(expr: String, data: String) -> String {
    let formula = formula::parse(&expr).unwrap();
    let table: Value = serde_json::from_str(&data).unwrap();
    return format!("{:?}", formula::eval(formula, &table)).to_string();
}
