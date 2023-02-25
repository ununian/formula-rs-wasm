#[cfg(test)]
mod formula_parse_ast {
    use formula_rs_wasm::{
        parse::{ast::to_ast, parse::Formula, to_operator::ToOperator},
        vm::{runner::Runner, value::Value},
    };

    #[test]
    fn vm_demo() {
        fn check(expr: &str, target: Value) {
            let formula = Formula::parse(expr).unwrap();
            let (_, ast) = to_ast(formula.paris);
            let operators = ast.to_operator();

            let runner = Runner;
            let result = runner.run(operators);

            println!("{:?}", result);

            assert_eq!(result.unwrap(), target);
        }

        check("1 + 2", Value::Number(3.into()));
        check("1+1*5", Value::Number(6.into()));
        check("1-1*5", Value::Number((-4).into()));
        check("(1+1)*5", Value::Number(10.into()));
        check("5!", Value::Number(120.into()));
    }
}
