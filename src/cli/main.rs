use formula_rs_wasm::{
    parse::{ast::to_ast, dependencies::get_dependencies, parse::Formula, to_operator::ToOperator},
    vm::{context::RuntimeContext, error::ExecuteError, runner::Runner, value::Value},
};
use hashbrown::{HashMap, HashSet};
use num::{FromPrimitive, Rational64};
use std::{
    fs, io,
    sync::{Arc, Mutex},
    time::{Instant, SystemTime},
};

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
    let issues = get_issues(&has_formula_issue_types, "prod_full");

    let ok_number = Arc::new(Mutex::new(0)); // @1
    let err_number = Arc::new(Mutex::new(0)); // @1
                                              // let err_count_map = Arc::new(Mutex::new(HashMap::<String, u64>::new())); // @1

    println!(
        "parse done, formula count {}, issue count {}, use {:?} ms",
        &byte_code_map.len(),
        &issues.len(),
        start.elapsed().as_millis()
    );

    issues.par_iter().for_each(|issue| {
        // issues.iter().for_each(|issue| {
        let issue_type_id = issue["issueTypeId"].as_i64().unwrap();

        let formula_field_codes = byte_code_map
            .keys()
            .into_iter()
            .filter(|(type_id, _)| *type_id == issue_type_id)
            .map(|(_, field_code)| field_code)
            .collect::<Vec<_>>();

        let runner = Runner;
        let mut context = RuntimeContext::new();
        context.inject_functions();
        mock_time(&mut context, issue);

        formula_field_codes.iter().for_each(|field_code| {
            let byte_code = byte_code_map
                .get(&(issue_type_id, field_code.to_string()))
                .unwrap();
            context.reset_stack();
            let result = run(byte_code, &runner, &mut context, &issue);

            match result {
                Ok(_) => {
                    let counter = Arc::clone(&ok_number);
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                }
                Err(_e) => {
                    let counter = Arc::clone(&err_number);
                    let mut num = counter.lock().unwrap();

                    // let hash_map = Arc::clone(&err_count_map);
                    // let mut map = hash_map.lock().unwrap();
                    // let err_count = map.entry(format!("{:?}", _e)).or_insert(0);
                    // *err_count += 1;

                    *num += 1;
                }
            }
        });
    });

    println!(
        "ok_number: {}, err_number: {}, use {:?} ms",
        *ok_number.lock().unwrap(),
        *err_number.lock().unwrap(),
        start.elapsed().as_millis()
    );

    let stdin = io::stdin();
    let _ = stdin.read_line(&mut String::new()).unwrap();

    // println!("err_set: {:#?}", *err_count_map.lock().unwrap());
}

fn get_formulas(env: &str) -> (HashMap<(i64, String), Vec<u8>>, HashSet<i64>) {
    let contents = fs::read_to_string(format!("data/{}_formula.json", env))
        .expect("Should have been able to read the file");

    let arr: Vec<FormulaItem> = serde_json::from_str(&contents).unwrap();

    let mut byte_code_map = HashMap::new();
    let mut has_formula_issue_types = HashSet::new();

    arr.iter()
        // .filter(|formula| formula.field_code == "customfield_48809227")
        .for_each(|formula| {
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
    let contents = fs::read_to_string(format!("data/{}_issues.json", env))
        .expect("Should have been able to read the file");

    let issues: Vec<JsonValue> = serde_json::from_str(&contents).unwrap();

    let issues_value = issues
        .into_iter()
        .filter(|issue| {
            issue["issueTypeId"].is_number()
                && has_formula_issue_types.contains(&issue["issueTypeId"].as_i64().unwrap())
            // && issue["id"].as_i64().unwrap() == 19828736
        })
        .collect::<Vec<JsonValue>>();

    issues_value
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
    runner: &Runner,
    ctx: &mut RuntimeContext,
    issue: &JsonValue,
) -> Result<Value, ExecuteError> {
    let operators = bincode::deserialize(&byte_code[..]).unwrap();
    let dependencies = get_dependencies(&operators);

    for dependency in dependencies {
        if ctx.has(&dependency) {
            continue;
        }
        let value = Value::from_json(&issue[&dependency]);
        // println!("dependency: {}", &dependency);
        ctx.set(dependency, value);
    }

    let result = runner.run(operators, ctx);

    result
}

fn mock_time(ctx: &mut RuntimeContext, issue: &JsonValue) {
    let now = SystemTime::now();
    let now = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let today = now - (now % 86400000);

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
