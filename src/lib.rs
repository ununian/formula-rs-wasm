#![no_std]

extern crate alloc;
use alloc::format;
use alloc::string::String;

#[macro_use]
extern crate pest_derive;

pub mod parse;
pub mod types;
pub mod execute;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(expr: String, _data: String) -> String {
    // let formula = formula::calc::parse(&expr).unwrap();
    // let table: Value = serde_json::from_str(&data).unwrap();
    // return format!("{}", formula::calc::eval(formula, &table));
    return format!("{}", expr);
}
