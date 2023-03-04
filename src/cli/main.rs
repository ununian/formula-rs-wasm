use formula_rs_wasm::{
    parse::{ast::to_ast, dependencies::get_dependencies, parse::Formula, to_operator::ToOperator},
    vm::{context::RuntimeContext, error::ExecuteError, runner::Runner, value::Value},
};
use hashbrown::{HashMap, HashSet};
use num::{FromPrimitive, Rational64};
use std::{fs, time::Instant};

use rayon::prelude::*;
use serde::{self, Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormulaItem {
    pub id: i64,
    pub name: String,
    pub expression: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub field_code: String,
    pub issue_type_id: i64,
}

fn main() {
    let start = Instant::now();

    let (byte_code_map, has_formula_issue_types) = get_formulas("prod");
    let mut issues = get_issues(&has_formula_issue_types, "prod_full");

    let mut ok_number = 0;
    let mut err_number = 1;

    println!(
        "parse done, formula count {}, issue count {}, use {:?} ms",
        &byte_code_map.len(),
        &issues.len(),
        start.elapsed().as_millis()
    );

    // issues.par_iter().for_each(|issue| {
    issues.iter().for_each(|issue| {
        let issue_type_id = issue["issueTypeId"].as_i64().unwrap();

        let formula_field_codes = byte_code_map
            .keys()
            .into_iter()
            .filter(|(type_id, _)| *type_id == issue_type_id)
            .map(|(_, field_code)| field_code)
            .collect::<Vec<_>>();

        formula_field_codes.iter().for_each(|field_code| {
            let byte_code = byte_code_map
                .get(&(issue_type_id, field_code.to_string()))
                .unwrap();

            let result = run(byte_code, field_code, issue);

            match result {
                Ok(_) => {
                    ok_number += 1;
                }
                Err(_) => {
                    err_number += 1;
                }
            }
        });
    });

    println!(
        "ok_number: {}, err_number: {}, use {:?} ms",
        ok_number,
        err_number,
        start.elapsed().as_millis()
    );
}

fn get_formulas(env: &str) -> (HashMap<(i64, String), Vec<u8>>, HashSet<i64>) {
    let contents = fs::read_to_string(format!("C:\\Users\\ununi\\Downloads\\{}_formula.json", env))
        .expect("Should have been able to read the file");

    let arr: Vec<FormulaItem> = serde_json::from_str(&contents).unwrap();

    let mut byte_code_map = HashMap::new();
    let mut has_formula_issue_types = HashSet::new();

    arr.iter().for_each(|formula| {
        let byte_code = compile(&formula.expression);
        has_formula_issue_types.insert(formula.issue_type_id);
        // key is issue_type_id + field_code
        byte_code_map.insert(
            (formula.issue_type_id, formula.field_code.to_string()),
            byte_code,
        );
    });

    (byte_code_map, has_formula_issue_types)
}

fn get_issues(has_formula_issue_types: &HashSet<i64>, env: &str) -> Vec<JsonValue> {
    let contents = fs::read_to_string(format!("C:\\Users\\ununi\\Downloads\\{}_issues.json", env))
        .expect("Should have been able to read the file");

    let issues: Vec<JsonValue> = serde_json::from_str(&contents).unwrap();

    let issues_value = issues
        .into_iter()
        .filter(|issue| {
            issue["issueTypeId"].is_number()
                && has_formula_issue_types.contains(&issue["issueTypeId"].as_i64().unwrap())
        })
        .collect::<Vec<JsonValue>>();

    issues_value
}

fn json_object_to_value(json: &JsonValue) -> Value {
    match json {
        JsonValue::Null => Value::String("".to_string()),
        JsonValue::Bool(b) => Value::Bool(*b),
        JsonValue::Number(n) => Value::Number(Rational64::from_f64(n.as_f64().unwrap()).unwrap()),
        JsonValue::String(s) => Value::String(s.to_string()),
        JsonValue::Array(arr) => {
            let mut vec = Vec::new();
            for item in arr {
                vec.push(json_object_to_value(item));
            }
            Value::Array(vec)
        }
        JsonValue::Object(obj) => {
            let mut map = HashMap::new();
            for (key, value) in obj {
                map.insert(key.to_string(), json_object_to_value(value));
            }
            Value::Object(map)
        }
    }
}

fn compile(expr: &str) -> Vec<u8> {
    let formula = Formula::parse(expr).unwrap();
    let (_, ast) = to_ast(formula.paris);
    let operators = ast.to_operator();
    let encoded: Vec<u8> = bincode::serialize(&operators).unwrap();
    encoded
}

fn run(
    byte_code: &Vec<u8>,
    _filed_code: &String,
    issue: &JsonValue,
) -> Result<Value, ExecuteError> {
    let operators = bincode::deserialize(&byte_code[..]).unwrap();
    let dependencies = get_dependencies(&operators);

    let runner = Runner;
    let mut context = RuntimeContext::new();
    context.inject_functions();
    mock_time(&mut context, issue);

    for dependency in dependencies {
        let value = json_object_to_value(&issue[&dependency]);
        // println!("dependency: {}", &dependency);
        context.set(dependency, value);
    }

    let result = runner.run(operators, &mut context);

    result
}

fn mock_time(ctx: &mut RuntimeContext, issue: &JsonValue) {
    ctx.set("GET_TODAY".to_string(), Value::Number(1677945600000.into()));
    ctx.set("GET_NOW".to_string(), Value::Number(1677946815406.into()));

    ctx.set(
        "GET_UPDATE_TIME".to_string(),
        json_object_to_value(&issue["updateTime"]),
    );

    ctx.set(
        "GET_CREATE_TIME".to_string(),
        json_object_to_value(&issue["createTime"]),
    );
}
