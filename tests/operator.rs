#[cfg(test)]
mod formula_parse_ast {
    use formula_rs_wasm::parse::to_operator::ToOperator;
    use formula_rs_wasm::parse::{ast::to_ast, parse::Formula};
    use formula_rs_wasm::share::operator::OperatorCode;

    #[test]
    fn encode() {
        let a = vec![
            OperatorCode::LoadIdentifier("a".to_string()),
            OperatorCode::PushString("123".to_string()),
            OperatorCode::Subtract,
        ];
        let encoded: Vec<u8> = bincode::serialize(&a).unwrap();
        println!("{:?}", encoded);
        let decoded: Vec<OperatorCode> = bincode::deserialize(&encoded[..]).unwrap();
        assert_eq!(a, decoded);
    }
}
