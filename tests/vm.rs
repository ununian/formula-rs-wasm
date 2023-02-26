#[cfg(test)]
mod formula_parse_ast {
    use formula_rs_wasm::{
        parse::{ast::to_ast, parse::Formula, to_operator::ToOperator},
        vm::{runner::Runner, value::Value},
    };
    use hashbrown::HashMap;

    #[test]
    fn vm_demo() {
        fn check(expr: &str, target: Value) {
            let formula = Formula::parse(expr).unwrap();
            let (_, ast) = to_ast(formula.paris);
            let operators = ast.to_operator();

            let runner = Runner;

            let mut heap = HashMap::new();

            heap.insert("a".to_string(), Value::Number(1.into()));
            heap.insert("b".to_string(), Value::String("abc".to_string()));

            let result = runner.run(operators, &mut heap);

            println!("{:?}", result);

            assert_eq!(result.unwrap(), target);
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

        check("a + 1", Value::Number(2.into()));
        check("b + 1", Value::String("abc1".to_string()));
    }
}
