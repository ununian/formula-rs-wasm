#[cfg(test)]
mod formula_type_check {
    use formula_rs_wasm::{
        parse::parse::Formula,
        types::{check::type_check, types::FormulaValueType},
    };

    #[test]
    fn add_success() {
        assert_eq!(
            type_check(&Formula::parse("1 + ( 2 + 3 ) + 3").unwrap().paris),
            Ok(FormulaValueType::Number)
        );

        assert_eq!(
            type_check(&Formula::parse("'123' + '321'").unwrap().paris),
            Ok(FormulaValueType::String)
        );

        assert_eq!(
            type_check(&Formula::parse("'123' + 2").unwrap().paris),
            Ok(FormulaValueType::String)
        );
    }
}
