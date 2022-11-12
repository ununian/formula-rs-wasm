#[cfg(test)]
mod formula_parse_ast {
    use formula_rs_wasm::parse::{ast::to_ast, parse::Formula};

    #[test]
    fn ast_demo() {
        let ast = to_ast(Formula::parse("'1' * 2").unwrap().paris);
        println!("{:?}", ast);
    }
}
