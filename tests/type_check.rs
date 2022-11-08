#[cfg(test)]
mod formula_type_check {
    use formula_rs_wasm::formula::{formula::Formula, type_check::type_check};

    #[test]
    fn add_number_number_success() {
        let result = type_check(&Formula::parse("1+1").unwrap().paris);
        assert!(result.is_ok());
    }
}
