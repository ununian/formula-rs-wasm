#[cfg(test)]
mod formula_parse_ast {
    use formula_rs_wasm::parse::{ast::to_ast, parse::Formula};

    #[test]
    fn ast_demo() {
        // let code = "1 + 2 * 3";
        // let code = "1";
        // let code = "a()";
        // let code = "a.b.c.d(1)";
        // let code = "a.b.c.d(1)";
        // let code = "a(1)";
        // let code = "a('1')";
        // let code = "a(1,2,3,'4')";
        let code = "a(1,2, 3+4,-2,'4' + '4',a)";
        let formula = Formula::parse(code).unwrap();
        // println!("{:#?}", formula);
        let (_, ast) = to_ast(formula.paris);
        println!("{:#?}", ast);
    }
}
