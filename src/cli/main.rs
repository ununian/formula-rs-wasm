use formula_rs_wasm::{
    parse::{ast::to_ast, parse::Formula, to_operator::ToOperator},
    vm::{context::RuntimeContext, error::ExecuteError, runner::Runner, value::Value},
};

fn main() {
    for _ in 0..2000000 {
        let byte_code = compile("1 + 2");
        let _ = run(&byte_code);
    }
}

fn run(byte_code: &Vec<u8>) -> Result<Value, ExecuteError> {
    let operators = bincode::deserialize(&byte_code[..]).unwrap();
    let runner = Runner;
    let mut context = RuntimeContext::new();
    context.inject_functions();
    let result = runner.run(operators, &mut context);

    result
}

fn compile(expr: &str) -> Vec<u8> {
    let formula = Formula::parse(expr).unwrap();
    let (_, ast) = to_ast(formula.paris);
    let operators = ast.to_operator();
    let encoded: Vec<u8> = bincode::serialize(&operators).unwrap();
    encoded
}
