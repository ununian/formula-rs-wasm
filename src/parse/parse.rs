extern crate pest;

use pest::{error::Error, iterators::Pairs, Parser};

#[derive(Parser)]
#[grammar = "parse/formula.pest"]
pub struct FormulaPest;

#[derive(Debug, Clone)]
pub struct Formula<'a> {
    pub paris: Pairs<'a, Rule>,
}

impl Formula<'_> {
    pub fn parse(input: &str) -> Result<Formula, Error<Rule>> {
        match FormulaPest::parse(Rule::formula, input) {
            Ok(pairs) => Ok(Formula { paris: pairs }),
            Err(e) => Err(e),
        }
    }
}
