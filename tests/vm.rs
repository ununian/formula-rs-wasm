#[cfg(test)]
mod formula_parse_ast {
    use formula_rs_wasm::{
        parse::{ast::to_ast, parse::Formula, to_operator::ToOperator},
        vm::{context::RuntimeContext, error::ExecuteError, runner::Runner, value::Value},
    };

    #[test]
    fn vm_demo() {
        fn run(expr: &str) -> Result<Value, ExecuteError> {
            let formula = Formula::parse(expr).unwrap();
            let (_, ast) = to_ast(formula.paris);
            let operators = ast.to_operator();

            let runner = Runner;

            let mut context = RuntimeContext::new();
            context.inject_functions();

            context.set("a".to_string(), Value::Number(2.into()));
            context.set("b".to_string(), Value::String("abc".to_string()));
            context.set(
                "arr".to_string(),
                Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]),
            );

            let result = runner.run(operators, &mut context);

            // println!("{:?}", result);
            result
        }

        fn check(expr: &str, target: Value) {
            assert_eq!(run(expr).unwrap(), target);
        }

        fn check_error(expr: &str, err: ExecuteError) {
            assert_eq!(run(expr), Err(err));
        }

        check("1 + 2", Value::Number(3.into()));
        check("1+1*5", Value::Number(6.into()));
        check("1-1*5", Value::Number((-4).into()));
        check("(1+1)*5", Value::Number(10.into()));
        check("(1+1)%5", Value::Number(2.into()));
        check("2^5", Value::Number(32.into()));
        check("5!", Value::Number(120.into()));

        check("'a' + 'b'", Value::String("ab".to_string()));
        check("1 + 'b'", Value::String("1b".to_string()));
        check("'a' + 1", Value::String("a1".to_string()));

        check("a + 1", Value::Number(3.into()));
        check("a ^ a", Value::Number(4.into()));
        check("b + 1", Value::String("abc1".to_string()));
        check_error(
            "c + 1",
            ExecuteError::identifier_not_found(&"c".to_string()),
        );

        check("SUM(1,2,3,4,5)", Value::Number(15.into()));
        check("SUM(arr)", Value::Number(3.into()));
        check_error(
            "SUM('arr')",
            ExecuteError::function_invalid_argument(vec!["Number", "Number[]"], vec!["String"]),
        );
    }
}
