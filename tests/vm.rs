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

            fn make_subtask(id: i64, estimate_point: i64, status: i64) -> Value {
                Value::Object(
                    [
                        ("id".to_string(), Value::Number(id.into())),
                        (
                            "estimatePoint".to_string(),
                            Value::Number(estimate_point.into()),
                        ),
                        ("status".to_string(), Value::Number(status.into())),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                )
            }

            fn make_relationship(id: i64, relationship: &str, issueType: i64) -> Value {
                Value::Object(
                    [
                        ("id".to_string(), Value::Number(id.into())),
                        (
                            "relationship".to_string(),
                            Value::String(relationship.to_string()),
                        ),
                        ("issueType".to_string(), Value::Number(issueType.into())),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                )
            }

            context.set(
                "subtask".to_string(),
                Value::Array(vec![
                    make_subtask(1, 1, 1),
                    make_subtask(2, 2, 1),
                    make_subtask(3, 3, 2),
                    make_subtask(4, 4, 2),
                ]),
            );

            context.set(
                "relationship".to_string(),
                Value::Array(vec![
                    make_relationship(1, "CHILD", 1),
                    make_relationship(2, "PARENT", 1),
                    make_relationship(3, "CHILD", 2),
                ]),
            );

            context.set("GET_NOW".to_string(), Value::DateTime(0));

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
        check("GET_NOW", Value::DateTime(0));
        check_error(
            "c + 1",
            ExecuteError::identifier_not_found(&"c".to_string()),
        );

        check(
            "arr",
            Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]),
        );

        check("SUM(1,2,3,4,5)", Value::Number(15.into()));
        check("SUM(arr)", Value::Number(3.into()));
        check_error(
            "SUM('arr')",
            ExecuteError::function_invalid_argument(vec!["Number", "Number[]"], vec!["String"]),
        );

        check_error(
            "SUM(subtask)",
            ExecuteError::function_invalid_argument(vec!["Number", "Number[]"], vec!["Array"]),
        );

        check(
            "subtask.estimatePoint",
            Value::Array(vec![
                Value::Number(1.into()),
                Value::Number(2.into()),
                Value::Number(3.into()),
                Value::Number(4.into()),
            ]),
        );

        check("COUNT(1,2,3,4,5)", Value::Number(5.into()));

        check("SUM(subtask.estimatePoint)", Value::Number(10.into()));
        check("COUNT(subtask)", Value::Number(4.into()));

        check("COUNT(subtask; status == 1)", Value::Number(2.into()));
        check("COUNT(relationship;relationship=CHILD)", Value::Number(2.into()));
    }
}
