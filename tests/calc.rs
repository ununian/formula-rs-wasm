extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};

#[cfg(test)]
mod parse_calc_tests {
    use formula_rs_wasm::formula::{self, *};
    use std::fs;

    #[test]
    fn parse_formulas() {
        let content = fs::read_to_string("tests/data/data.txt").unwrap();

        for formula in content.lines() {
            let result = formula::parse(&formula);
            match result {
                Err(err) => {
                    assert!(false, "{} Failed: {}", formula, err);
                }
                _ => (),
            }
        }
    }

    #[test]
    fn parse_get_function() {
        let formula = "COUNT(relationship;)";
        let result = formula::parse(&formula).unwrap();

        // println!("{:?}", result);
        let function =
            Function::from(result.clone().next().unwrap().into_inner().next().unwrap()).unwrap();
        assert_eq!(function.name, "COUNT");
        assert_eq!(
            function.parts,
            vec![formula::FunctionPart::Identifier(
                "relationship".to_string()
            )]
        );
    }

    #[test]
    fn parse_get_function_with_compare() {
        let formula = "COUNT(relationship;issueTypeId=1848788)";
        let result = formula::parse(&formula).unwrap();

        let function =
            Function::from(result.clone().next().unwrap().into_inner().next().unwrap()).unwrap();
        assert_eq!(function.name, "COUNT");
        assert_eq!(
            function.parts,
            vec![
                FunctionPart::Identifier("relationship".to_string()),
                FunctionPart::CompareExpression(
                    "issueTypeId".to_string(),
                    CompareOperator::Equal,
                    "1848788".to_string()
                )
            ]
        );
    }

    #[test]
    fn parse_get_function_with_compare_2() {
        let formula = "SUM(subtask.estimatePoint;status=4)";
        let result = formula::parse(&formula).unwrap();

        let function =
            Function::from(result.clone().next().unwrap().into_inner().next().unwrap()).unwrap();
        assert_eq!(function.name, "SUM");
        assert_eq!(
            function.parts,
            vec![
                FunctionPart::IdentifierWithField(
                    "subtask".to_string(),
                    "estimatePoint".to_string()
                ),
                FunctionPart::CompareExpression(
                    "status".to_string(),
                    CompareOperator::Equal,
                    "4".to_string()
                )
            ]
        );
    }

    #[test]
    fn parse_expr() {
        let formula = "SUM(subtask.estimatePoint;status=2) + GET_NOW-GET_UPDATE_TIME";
        let result = formula::parse(&formula).unwrap();

        let expr = Expression::from_pairs(result);

        assert!(expr.is_some())
    }

    #[test]
    fn get_expr_dependencies_1() {
        let formula = "GET_NOW-GET_UPDATE_TIME";
        let result = formula::parse(&formula).unwrap();
        let expr = Expression::from_pairs(result).unwrap();

        let dependencies = expr.get_dependencies();

        assert_eq!(
            dependencies
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>(),
            vec!["GET_NOW", "GET_UPDATE_TIME",]
        );
    }

    #[test]
    fn get_expr_dependencies_2() {
        let formula = "SUM(subtask.estimatePoint;status=2) + GET_NOW-GET_UPDATE_TIME";
        let result = formula::parse(&formula).unwrap();
        let expr = Expression::from_pairs(result).unwrap();

        let dependencies = expr.get_dependencies();

        assert_eq!(
            dependencies
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>(),
            vec!["subtask", "GET_NOW", "GET_UPDATE_TIME",]
        );
    }
}

#[cfg(test)]
mod number_calc_tests {
    use formula_rs_wasm::formula::{self, *};

    use serde_json::{json, Value};
    fn create_num_table() -> Value {
        json!({
            "a": 6,
            "b": 3,
            "c": 5,
            "d": 8,
        })
    }

    #[test]
    fn calc_add() {
        let exp = formula::parse("a + b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(9.0));
    }

    #[test]
    fn calc_add_multi() {
        let result = formula::eval(
            formula::parse("a + b + c + d").unwrap(),
            &create_num_table(),
        );

        let result2 = formula::eval(formula::parse("d+c+a+b").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(22.0));
        assert_eq!(result2, ExpValue::Number(22.0));
    }

    #[test]
    fn calc_add_assoc() {
        let result = formula::eval(formula::parse("a + (b + c)").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(a + b) + c").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(14.0));
        assert_eq!(result2, ExpValue::Number(14.0));
    }

    #[test]
    fn calc_sub() {
        let exp = formula::parse("a - b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(3.0));
    }

    #[test]
    fn calc_sub_multi() {
        let result = formula::eval(
            formula::parse("a - b - c - d").unwrap(),
            &create_num_table(),
        );
        assert_eq!(result, ExpValue::Number(-10.0));
    }

    #[test]
    fn calc_sub_assoc() {
        let result = formula::eval(formula::parse("a - (b - c)").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(a - b) - c").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(8.0));
        assert_eq!(result2, ExpValue::Number(-2.0));
    }

    #[test]
    fn calc_negation_add() {
        let exp = formula::parse("1 + -1").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(0.0));
    }

    #[test]
    fn calc_negation_sub() {
        let exp = formula::parse("1 - -1").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(2.0));
    }

    #[test]
    fn calc_negation_mul() {
        let exp = formula::parse("1 * -1").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(-1.0));
    }

    #[test]
    fn calc_negation_div() {
        let exp = formula::parse("1 / -1").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(-1.0));
    }

    #[test]
    fn calc_negation_pow() {
        let exp = formula::parse("1 ^ -1").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(1.0));
    }

    #[test]
    fn calc_negation_pow_2() {
        let exp = formula::parse("1 ^ -2").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(1.0));
    }


    #[test]
    fn calc_negation_pow_3() {
        let exp = formula::parse("2 ^ -1").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(0.5));
    }

    #[test]
    fn calc_negation_pow_4() {
        let exp = formula::parse("2 ^ -2").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(0.25));
    }

    #[test]
    fn calc_mul() {
        let exp = formula::parse("a * b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(18.0));
    }

    #[test]
    fn calc_mul_multi() {
        let result = formula::eval(
            formula::parse("a * b * c * d").unwrap(),
            &create_num_table(),
        );
        assert_eq!(result, ExpValue::Number(720.0));
    }

    #[test]
    fn calc_mul_assoc() {
        let result = formula::eval(formula::parse("a * (b * c)").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(a * b) * c").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(90.0));
        assert_eq!(result2, ExpValue::Number(90.0));
    }

    #[test]
    fn calc_div() {
        let exp = formula::parse("a / b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(2.0));
    }

    #[test]
    fn calc_div_multi() {
        let result = formula::eval(
            formula::parse("a / b / c / d").unwrap(),
            &create_num_table(),
        );
        assert_eq!(result, ExpValue::Number(0.05));
    }

    #[test]
    fn calc_div_assoc() {
        let result = formula::eval(formula::parse("d / (a / b)").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(a / b) / c").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(4.0));
        assert_eq!(result2, ExpValue::Number(0.4));
    }

    #[test]
    fn calc_pow() {
        let exp = formula::parse("a ^ b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(216.0));
    }

    #[test]
    fn calc_pow_multi() {
        let result = formula::eval(formula::parse("4 ^ 3 ^ 2").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(262144.0));
    }

    #[test]
    fn calc_pow_assoc() {
        let result = formula::eval(formula::parse("4 ^ 3 ^ 2").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(4 ^ 3) ^ 2").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(262144.0));
        assert_eq!(result2, ExpValue::Number(4096.0));
    }

    #[test]
    fn calc_rem() {
        let exp = formula::parse("a % b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(0.0));
    }

    #[test]
    fn calc_factorial_0() {
        let exp = formula::parse("0!").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(1.0));
    }

    #[test]
    fn calc_factorial_1() {
        let exp = formula::parse("1!").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(1.0));
    }

    #[test]
    fn calc_factorial_5() {
        let exp = formula::parse("5!").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(120.0));
    }

    #[test]
    fn calc_factorial_5_5() {
        let exp = formula::parse("5.5!").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(120.0));
    }

    #[test]
    fn calc_factorial_neg_5() {
        let exp = formula::parse("-5!").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(-120.0));
    }

    #[test]
    fn calc_factorial_add_5() {
        let exp = formula::parse("5! + 5!").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(240.0));
    }
}

#[cfg(test)]
mod pass_value_test {

    use alloc::string::ToString;
    use formula_rs_wasm::formula::{self, *};

    use serde_json::json;

    #[test]
    fn pass_number() {
        let table = json!({
            "a": 6,
        });

        let exp = formula::parse("a").unwrap();
        let result = formula::eval(exp, &table);
        assert_eq!(result, ExpValue::Number(6.0));
    }

    #[test]
    fn pass_string() {
        let table = json!({
            "a": "123123",
        });

        let exp = formula::parse("a").unwrap();
        let result = formula::eval(exp, &table);
        assert_eq!(result, ExpValue::String("123123".to_string()));
    }
}

#[cfg(test)]
mod function_test {

    use formula_rs_wasm::formula::{self, *};

    use alloc::vec;
    use serde_json::{json, Value};
    fn get_data() -> Value {
        json!({
            "a": 3,
            "estimatePoint": 10,
            "subtask": [
                {
                    "id": 1,
                    "name": "test1",
                    "status": 2,
                    "estimatePoint": 1,
                },
                {
                    "id": 2,
                    "name": "test2",
                    "status": 2,
                    "estimatePoint": 2,
                },
                {
                    "id": 3,
                    "name": "test3",
                    "status": 3,
                    "estimatePoint": 3,
                },
                {
                    "id": 4,
                    "name": "test4",
                    "status": 3,
                    "estimatePoint": 4,
                },
            ]
        })
    }

    #[test]
    fn func_run() {
        let exp = formula::parse("SUM(subtask;status=2)").unwrap();
        let func = match Expression::from_pairs(exp).unwrap().parts[0].clone() {
            ExpressionPart::Function(f) => f,
            _ => panic!("not function"),
        };
        let sum = func.run(&get_data());
        assert_eq!(sum, Some(ExpValue::Number(0.0)));
    }

    #[test]
    fn func_run_with_exp() {
        let exp = formula::parse("SUM(subtask.estimatePoint;status=2) + a").unwrap();
        let sum = formula::eval(exp, &get_data());
        assert_eq!(sum, ExpValue::Number(6.0));
    }

    #[test]
    fn func_run_with_exp_mul() {
        let exp = formula::parse("SUM(subtask.estimatePoint;status=2) * a").unwrap();
        let sum = formula::eval(exp, &get_data());
        assert_eq!(sum, ExpValue::Number(9.0));
    }

    #[test]
    fn func_run_with_field() {
        let exp = formula::parse("SUM(subtask.estimatePoint;status=2) + estimatePoint").unwrap();
        let sum = formula::eval(exp, &get_data());
        assert_eq!(sum, ExpValue::Number(13.0));

        let exp = formula::parse(
            "estimatePoint ^ COUNT(subtask.estimatePoint;status=3) + SUM(subtask.estimatePoint;status=2)"
        ).unwrap();
        let sum = formula::eval(exp, &get_data());
        assert_eq!(sum, ExpValue::Number(103.0));
    }

    #[test]
    fn func_run_with_json_string() {
        let json = r#"{
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

        let data: Value = serde_json::from_str(json).unwrap();

        let exp = formula::parse(
            "estimatePoint ^ COUNT(subtask.estimatePoint;status=3) + SUM(subtask.estimatePoint;status=2)"
        ).unwrap();
        let sum = formula::eval(exp, &data);
        assert_eq!(sum, ExpValue::Number(103.0));
    }
}
