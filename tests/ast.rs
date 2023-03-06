#[cfg(test)]
mod formula_parse_ast {
    use expect_test::{expect, Expect};
    use formula_rs_wasm::parse::beautify::Beautify;
    use formula_rs_wasm::parse::dependencies::get_dependencies;
    use formula_rs_wasm::parse::to_operator::ToOperator;
    use formula_rs_wasm::parse::{ast::to_ast, parse::Formula};
    use formula_rs_wasm::share::operator::OperatorCode;

    fn check_ast(expr: &str, expected: Expect) {
        let formula = Formula::parse(expr).unwrap();
        let (_, ast) = to_ast(formula.paris);
        let actual = ast.beautify(0);

        expected.assert_eq(actual.as_str());

        // let actual_lines = actual.lines().filter(|s| s.trim().len() > 0);
        // let expected_lines = expected.lines().filter(|s| s.trim().len() > 0);

        // for (actual_line, expected_line) in actual_lines.zip(expected_lines) {
        //     let result = actual_line.trim() == expected_line.trim();
        //     assert!(result, "actual: \n{}, \nexpected: \n{}\n", actual, expected);
        //     if !result {
        //         break;
        //     }
        // }
    }

    #[test]
    fn ast_demo() {
        let code = "(1 + 1) * 3";
        // let code = "2!";
        // let code = "a()";
        // let code = "a.b";
        // let code = "a.b.c.d(1)";
        // let code = "a(1)";
        // let code = "a('1')";
        // let code = "a(1,2,3,'4')";
        // let code = "2 > 1";
        // let code = "a.b.c(1,2, 3+4,-2,'4' + '4',a)";
        // let code = "a(a())";
        // let code = "a(b(c(d(1))))";
        // let code = "5! * count(where(subtask,$.updateTime > now(aa.a + 2)))";
        // let code = "type NewType = { a: Number, b: Array<Number>, c: { d: { e: Bool } } };";
        // let code = "type a = Number;";
        let code = "SUM(subtask.estimatePoint;status=4)/SUM(subtask.estimatePoint;)";
        let formula = Formula::parse(code).unwrap();
        // println!("{:#?}", formula);
        let (_, ast) = to_ast(formula.paris);
        // println!("{:#?}", ast);
        println!("{}", ast.beautify(0));

        let operators = ast.to_operator();
        println!("{:#?}", operators);

    }

    #[test]
    fn ast_test() {
        check_ast(
            "1.2",
            expect![
                r#"
            FormulaBody
                ExpressionStatement
                    NumberLiteral (6/5)"#
            ],
        );

        check_ast(
            "1 + 1 * 3",
            expect![
                r#"
                FormulaBody
                    ExpressionStatement
                        BinaryExpression
                            left
                                NumberLiteral (1)
                            operator +
                            right
                                BinaryExpression
                                    left
                                        NumberLiteral (1)
                                    operator *
                                    right
                                        NumberLiteral (3)"#
            ],
        );

        check_ast(
            "sum(arr, point > 2)",
            expect![
                r#"
                FormulaBody
                    ExpressionStatement
                        CallExpression
                            callee
                                Identifier sum
                            arguments
                                Identifier arr
                                BinaryExpression
                                    left
                                        Identifier point
                                    operator >
                                    right
                                        NumberLiteral (2)"#
            ],
        );

        check_ast(
            "(1 + 1) * 3",
            expect![
                r#"
                FormulaBody
                    ExpressionStatement
                        BinaryExpression
                            left
                                BinaryExpression
                                    left
                                        NumberLiteral (1)
                                    operator +
                                    right
                                        NumberLiteral (1)
                            operator *
                            right
                                NumberLiteral (3)"#
            ],
        );

        // TODO: 因为奇怪的过滤表达式，导致这个测试无法通过
        // check_ast(
        //     "5! * count(where(subtask,$.updateTime > now(aa.a + 2)))",
        //     expect![[r#"
        //         FormulaBody
        //             ExpressionStatement
        //                 BinaryExpression
        //                     left
        //                         UnaryExpression
        //                             operator !
        //                             argument
        //                                 NumberLiteral (5)
        //                     operator *
        //                     right
        //                         CallExpression
        //                             callee
        //                                 Identifier count
        //                             arguments
        //                                 CallExpression
        //                                     callee
        //                                         Identifier where
        //                                     arguments
        //                                         Identifier subtask
        //                                         BinaryExpression
        //                                             left
        //                                                 PropertyAccessExpression
        //                                                     object
        //                                                         Identifier $
        //                                                     property
        //                                                         Identifier updateTime
        //                                             operator >
        //                                             right
        //                                                 CallExpression
        //                                                     callee
        //                                                         Identifier now
        //                                                     arguments
        //                                                         BinaryExpression
        //                                                             left
        //                                                                 PropertyAccessExpression
        //                                                                     object
        //                                                                         Identifier aa
        //                                                                     property
        //                                                                         Identifier a
        //                                                             operator +
        //                                                             right
        //                                                                 NumberLiteral (2)"#]],
        // );

        check_ast(
            "2!",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    UnaryExpression
                        operator !
                        argument
                            NumberLiteral (2)"#]],
        );

        check_ast(
            "a()",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    CallExpression
                        callee
                            Identifier a
                        arguments
                            (EMPTY)"#]],
        );

        check_ast(
            "a.b",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    PropertyAccessExpression
                        object
                            Identifier a
                        property
                            Identifier b"#]],
        );

        check_ast(
            "a.b.c.d(1)",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    CallExpression
                        callee
                            PropertyAccessExpression
                                object
                                    PropertyAccessExpression
                                        object
                                            PropertyAccessExpression
                                                object
                                                    Identifier a
                                                property
                                                    Identifier b
                                        property
                                            Identifier c
                                property
                                    Identifier d
                        arguments
                            NumberLiteral (1)"#]],
        );

        check_ast(
            "a('1')",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    CallExpression
                        callee
                            Identifier a
                        arguments
                            StringLiteral ('1')"#]],
        );

        check_ast(
            "a(1,2,3,'4')",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    CallExpression
                        callee
                            Identifier a
                        arguments
                            NumberLiteral (1)
                            NumberLiteral (2)
                            NumberLiteral (3)
                            StringLiteral ('4')"#]],
        );

        // TODO: 因为奇怪的过滤表达式，导致这个测试无法通过
        // check_ast(
        //     "a.b.c(1,2, 3+4,-2,'4' + '4',a + 1, a> 2)",
        //     expect![[r#"
        //     FormulaBody
        //         ExpressionStatement
        //             CallExpression
        //                 callee
        //                     PropertyAccessExpression
        //                         object
        //                             PropertyAccessExpression
        //                                 object
        //                                     Identifier a
        //                                 property
        //                                     Identifier b
        //                         property
        //                             Identifier c
        //                 arguments
        //                     NumberLiteral (1)
        //                     NumberLiteral (2)
        //                     BinaryExpression
        //                         left
        //                             NumberLiteral (3)
        //                         operator +
        //                         right
        //                             NumberLiteral (4)
        //                     NumberLiteral (-2)
        //                     BinaryExpression
        //                         left
        //                             StringLiteral ('4')
        //                         operator +
        //                         right
        //                             StringLiteral ('4')
        //                     BinaryExpression
        //                         left
        //                             Identifier a
        //                         operator +
        //                         right
        //                             NumberLiteral (1)
        //                     BinaryExpression
        //                         left
        //                             Identifier a
        //                         operator >
        //                         right
        //                             NumberLiteral (2)"#]],
        // );

        check_ast(
            "2 > 1",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    BinaryExpression
                        left
                            NumberLiteral (2)
                        operator >
                        right
                            NumberLiteral (1)"#]],
        );

        check_ast(
            "a > b",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    BinaryExpression
                        left
                            Identifier a
                        operator >
                        right
                            Identifier b"#]],
        );

        check_ast(
            "a(b(c(d(1))))",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    CallExpression
                        callee
                            Identifier a
                        arguments
                            CallExpression
                                callee
                                    Identifier b
                                arguments
                                    CallExpression
                                        callee
                                            Identifier c
                                        arguments
                                            CallExpression
                                                callee
                                                    Identifier d
                                                arguments
                                                    NumberLiteral (1)"#]],
        );

        check_ast(
            "1+1;2+1",
            expect![[r#"
        FormulaBody
            ExpressionStatement
                BinaryExpression
                    left
                        NumberLiteral (1)
                    operator +
                    right
                        NumberLiteral (1)
            ExpressionStatement
                BinaryExpression
                    left
                        NumberLiteral (2)
                    operator +
                    right
                        NumberLiteral (1)"#]],
        );

        check_ast(
            "type NewType = { a: Number, b: Array<Number>, c: { d: { e: Bool } } };",
            expect![[r#"
            FormulaBody
                ExpressionStatement
                    TypeDefine NewType is { a: Number, b: Array<Number>, c: { d: { e: Bool } } }"#]],
        );
    }

    #[test]
    fn ast_to_operator() {
        fn check(expr: &str, result: Vec<OperatorCode>) {
            let formula = Formula::parse(expr).unwrap();
            let (_, ast) = to_ast(formula.paris);

            let operators = ast.to_operator();
            assert_eq!(operators, result);

            // println!("{:#?}", get_dependencies(&operators))
        }

        check(
            "5!",
            vec![OperatorCode::PushNumber(5.into()), OperatorCode::Factorial],
        );

        check(
            "1 + 3",
            vec![
                OperatorCode::PushNumber(1.into()),
                OperatorCode::PushNumber(3.into()),
                OperatorCode::Add,
            ],
        );

        check(
            "1 + 3 * 5",
            vec![
                OperatorCode::PushNumber(1.into()),
                OperatorCode::PushNumber(3.into()),
                OperatorCode::PushNumber(5.into()),
                OperatorCode::Multiply,
                OperatorCode::Add,
            ],
        );

        check(
            "(1 + 3) * 5",
            vec![
                OperatorCode::PushNumber(1.into()),
                OperatorCode::PushNumber(3.into()),
                OperatorCode::Add,
                OperatorCode::PushNumber(5.into()),
                OperatorCode::Multiply,
            ],
        );

        check(
            "1 - 1",
            vec![
                OperatorCode::PushNumber(1.into()),
                OperatorCode::PushNumber(1.into()),
                OperatorCode::Subtract,
            ],
        );

        check(
            "a - 1",
            vec![
                OperatorCode::LoadIdentifier("a".to_string()),
                OperatorCode::PushNumber(1.into()),
                OperatorCode::Subtract,
            ],
        );

        check(
            "a - b",
            vec![
                OperatorCode::LoadIdentifier("a".to_string()),
                OperatorCode::LoadIdentifier("b".to_string()),
                OperatorCode::Subtract,
            ],
        );

        check(
            "a - '123'",
            vec![
                OperatorCode::LoadIdentifier("a".to_string()),
                OperatorCode::PushString("123".to_string()),
                OperatorCode::Subtract,
            ],
        );

        check(
            "a(1)",
            vec![
                OperatorCode::LoadIdentifier("a".to_string()),
                OperatorCode::PushNumber(1.into()),
                OperatorCode::Call(1),
            ],
        );

        // TODO: 目前仅需要支持 过滤表达式
        // check(
        //     "a(a(1 + 1))",
        //     vec![
        //         OperatorCode::LoadIdentifier("a".to_string()),
        //         OperatorCode::LoadIdentifier("a".to_string()),
        //         OperatorCode::PushNumber(1.into()),
        //         OperatorCode::PushNumber(1.into()),
        //         OperatorCode::Add,
        //         OperatorCode::Call(1),
        //         OperatorCode::Call(1),
        //     ],
        // );

        check(
            "a(1,2,3,4)",
            vec![
                OperatorCode::LoadIdentifier("a".to_string()),
                OperatorCode::PushNumber(1.into()),
                OperatorCode::PushNumber(2.into()),
                OperatorCode::PushNumber(3.into()),
                OperatorCode::PushNumber(4.into()),
                OperatorCode::Call(4),
            ],
        );

        check(
            "SUM(arr;)",
            vec![
                OperatorCode::LoadIdentifier("SUM".to_string()),
                OperatorCode::LoadIdentifier("arr".to_string()),
                OperatorCode::Call(1),
            ],
        );

        check(
            "SUM(subtask.estimatePoint)",
            vec![
                OperatorCode::LoadIdentifier("SUM".to_string()),
                OperatorCode::LoadIdentifier("subtask".to_string()),
                OperatorCode::LoadPropertyAccess("estimatePoint".to_string()),
                OperatorCode::Call(1),
            ],
        );

        check(
            "SUM(subtask.estimatePoint; state == 1)",
            vec![
                OperatorCode::LoadIdentifier("SUM".to_string()),
                OperatorCode::LoadIdentifier("subtask".to_string()),
                OperatorCode::FilterExpression(
                    "state".to_string(),
                    "==".to_string(),
                    "1".to_string(),
                ),
                OperatorCode::LoadPropertyAccess("estimatePoint".to_string()),
                OperatorCode::Call(1),
            ],
        );

        check(
            "COUNT(relationship;relationship=CHILD)",
            vec![
                OperatorCode::LoadIdentifier("COUNT".to_string()),
                OperatorCode::LoadIdentifier("relationship".to_string()),
                OperatorCode::FilterExpression(
                    "relationship".to_string(),
                    "=".to_string(),
                    "CHILD".to_string(),
                ),
                OperatorCode::Call(1),
            ],
        );

        
    }
}
