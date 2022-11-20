#[cfg(test)]
mod formula_parse_ast {
    use formula_rs_wasm::parse::{ast::to_ast, parse::Formula};
    use formula_rs_wasm::parse::beautify::Beautify;

    #[test]
    fn ast_demo() {
        // let code = "1 + 1 * 3";
        // let code = "2!";
        // let code = "a()";
        // let code = "a.b";
        // let code = "a.b.c.d(1)";
        // let code = "a(1)";
        // let code = "a('1')";
        // let code = "a(1,2,3,'4')";
        // let code = "5! * count(where(subtask,$.updateTime > now(aa.a + 2)))";
        // let code = "a.b.c(1,2, 3+4,-2,'4' + '4',a)";
        // let code = "a(a())";
        let code = "a(b(c(d(1))))";
        let formula = Formula::parse(code).unwrap();
        // println!("{:#?}", formula);
        let (_, ast) = to_ast(formula.paris);
        println!("{:#?}", ast);
        println!("{}", ast.beautify(0));
    }
}
