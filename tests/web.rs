//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use formula_rs_wasm::{formula::ExpValue, greet};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn greet_test() {
    let data = r#"{
        "a": 3,
        "estimatePoint": 10,
        "subtask": [
          {
            "id": 1,
            "name": "test1",
            "status": 2,
            "estimatePoint": 1
          },
          {
            "id": 2,
            "name": "test2",
            "status": 2,
            "estimatePoint": 2
          },
          {
            "id": 3,
            "name": "test3",
            "status": 3,
            "estimatePoint": 3
          },
          {
            "id": 4,
            "name": "test4",
            "status": 3,
            "estimatePoint": 4
          }
        ]
      }
      "#;
    let r = greet("estimatePoint ^ COUNT(subtask.estimatePoint;status=3) + SUM(subtask.estimatePoint;status=2)".to_string(), data.to_string());
    assert_eq!(r, "Number(103.0)");
}
