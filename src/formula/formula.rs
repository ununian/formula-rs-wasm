extern crate pest;

use pest::{error::Error, iterators::Pairs, Parser};

#[derive(Parser)]
#[grammar = "formula/formula.pest"]
pub struct FormulaPest;

#[derive(Debug, Clone)]
pub struct Formula<'a> {
    pub rules: Pairs<'a, Rule>,
}

impl Formula<'_> {
    pub fn parse(input: &str) -> Result<Formula, Error<Rule>> {
        match FormulaPest::parse(Rule::formula, input) {
            Ok(pairs) => Ok(Formula { rules: pairs }),
            Err(e) => Err(e),
        }
    }
}
