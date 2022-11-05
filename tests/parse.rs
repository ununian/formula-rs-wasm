use formula_rs_wasm::formula::formula::{Formula, Rule};
use pest::{error::Error, iterators::Pairs};

extern crate alloc;

/*
Error 中 positives 是期待某个 XXX 的规则，negatives 是不期待某个 XXX 的规则
*/

fn get_rules(result: Result<Formula, Error<Rule>>) -> Pairs<Rule> {
    if result.is_err() {
        println!("result: {:?}", result);
    }
    assert!(result.is_ok());
    let formula = result.unwrap();
    formula.rules
}

// fn get_rules_with_log(result: Result<Formula, Error<Rule>>) -> Pairs<Rule> {
//     println!("result: {:?}", result);
//     get_rules(result)
// }

fn match_expr_rules(mut rules: Pairs<Rule>, target: Vec<Rule>) {
    let expr = rules.next().unwrap();
    assert_eq!(expr.as_rule(), Rule::expr);
    let mut expr_rules = expr.into_inner();
    for target_rule in target {
        let rule = expr_rules.next();
        assert!(rule.is_some());
        assert_eq!(rule.unwrap().as_rule(), target_rule);
    }
}

fn match_rules(mut rules: Pairs<Rule>, target: Vec<Rule>) {
    for target_rule in target {
        let rule = rules.next();
        assert!(rule.is_some());
        assert_eq!(rule.unwrap().as_rule(), target_rule);
    }
}

#[cfg(test)]
mod formula_parse_literal_num {

    use formula_rs_wasm::formula::formula::{Formula, Rule};

    use crate::get_rules;

    #[test]
    fn num_allow_value() {
        vec![
            "1", "1.1", "0", "+0", "2.2", "2.0", "2e20", "-1", "+1", "-1.1", "+1.1", "-1.0",
            "+1.0", "-1e20", "+1e20",
        ]
        .iter()
        .for_each(|s| {
            let mut rules = get_rules(Formula::parse(s));
            assert_eq!(rules.clone().count(), 2);
            assert_eq!(rules.next().unwrap().as_rule(), Rule::num);
        });
    }

    #[test]
    fn num_illegal_value() {
        vec![
            "1.", "0.", "0..1", "1.1.1", "++", "1e", "1e+", "1e-", "1e-+", "1e++", "1e--", "1e+-",
            "+-1", "-+1",
        ]
        .iter()
        .for_each(|s| {
            let result = Formula::parse(s);
            assert!(result.is_err());
        });
    }
}

#[cfg(test)]
mod formula_parse_literal_string {
    use formula_rs_wasm::formula::formula::{Formula, Rule};

    use crate::{get_rules, match_rules};

    #[test]
    fn string_allow_value() {
        r#"
        ""
        "abc"
        "123'asd'123123"
        "123\n"
        "$我是中文"
        "1997-01-24"
        "”“"
        "::Ok"
        "😀✨😃🎃😄🔥😁"
        "#
        .lines()
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .for_each(|s| {
            let rules = get_rules(Formula::parse(s));
            assert_eq!(rules.clone().count(), 2);
            match_rules(rules, vec![Rule::string]);
        });
    }

    #[test]
    fn string_illegal_value() {
        vec!["\"", "\"\"\""].iter().for_each(|s| {
            let result = Formula::parse(s);
            assert!(result.is_err());
        });
    }
}

#[cfg(test)]
mod formula_parse_identifier {
    use formula_rs_wasm::formula::formula::{Formula, Rule};

    use crate::{get_rules, match_rules};

    #[test]
    fn identifier_allow_value() {
        vec!["a", "_a", "a1", "我的变量", "我12", "$", "$$"]
            .iter()
            .for_each(|s| {
                let rules = get_rules(Formula::parse(s));
                assert_eq!(rules.clone().count(), 2);
                match_rules(rules, vec![Rule::identifier]);
            });
    }

    #[test]
    fn identifier_illegal_value() {
        vec!["1a", "1_", "%a"].iter().for_each(|s| {
            let result = Formula::parse(s);
            println!("result: {:?}", result);
            assert!(result.is_err());
        });
    }

    #[test]
    fn identifier_illegal_value_type() {
        vec!["1", "-1"].iter().for_each(|s| {
            let mut rules = get_rules(Formula::parse(s));
            assert_eq!(rules.clone().count(), 2);
            assert_ne!(rules.next().unwrap().as_rule(), Rule::identifier);
        });
    }
}

#[cfg(test)]
mod formula_parse_operation {
    use formula_rs_wasm::formula::formula::{Formula, Rule};

    use crate::get_rules;

    #[test]
    fn operation_allow_value() {
        vec![
            "1 + 1",
            "a*x",
            "a/1",
            "2 ^ 1",
            "(1 + a) / 2",
            "4!",
            "-1! + 2",
            "2 ^ -2",
            "2 >> 1",
        ]
        .iter()
        .for_each(|s| {
            let mut rules = get_rules(Formula::parse(s));
            assert_eq!(rules.next().unwrap().as_rule(), Rule::operation_expr);
        });
    }
}

#[cfg(test)]
mod formula_parse_dot {
    use formula_rs_wasm::formula::formula::{Formula, Rule};

    use crate::{get_rules, match_rules};

    #[test]
    fn dot_allow_value() {
        vec!["a.a", "$.a"].iter().for_each(|s| {
            let rules = get_rules(Formula::parse(s));
            assert_eq!(rules.clone().count(), 3);
            match_rules(rules, vec![Rule::identifier, Rule::dot]);
        });

        vec!["a.a.a.a"].iter().for_each(|s| {
            let rules = get_rules(Formula::parse(s));
            assert_eq!(rules.clone().count(), 5);
            match_rules(rules, vec![Rule::identifier, Rule::dot]);
        });
    }
}

#[cfg(test)]
mod formula_parse_compare {
    use formula_rs_wasm::formula::formula::{Formula, Rule};

    use crate::{get_rules, match_rules};

    #[test]
    fn compare_allow_value() {
        vec![
            "1 == 1",
            "1 != 1",
            " a > a - 1",
            "$.name < bb",
            "$name != 123",
            "($name.a + 1) != 123",
        ]
        .iter()
        .for_each(|s| {
            let rules = get_rules(Formula::parse(s));
            match_rules(rules, vec![Rule::compare_expr]);
        });
    }
}

#[cfg(test)]
mod formula_parse_function {
    use formula_rs_wasm::formula::formula::{Formula, Rule};

    use crate::{get_rules, match_rules};

    #[test]
    fn function_allow_value() {
        vec![
            "where()",
            "count(where(subtask, $.updateTime > now()))",
            "count( where( subtask, $.updateTime > now(aa.a + 2) ) )",
            "count( where( subtask, $.updateTime > (now() + day(1) ) ) )",
        ]
        .iter()
        .for_each(|s| {
            let rules = get_rules(Formula::parse(s));
            match_rules(rules, vec![Rule::function]);
        });

        vec![
            "count( where( subtask, $.updateTime > (now(aa.a + 2) + day(1) ) ) )",
        ]
        .iter()
        .for_each(|s| {
            let rules = get_rules(Formula::parse(s));
            println!("rules: {:?}", rules.clone().collect::<Vec<_>>());
            match_rules(rules, vec![Rule::function]);
        });

        vec![
            "1+count()",
            "5! * count(where(subtask,$.updateTime > now(aa.a + 2)))",
            "now() + day(1) + hour(1)"
        ]
        .iter()
        .for_each(|s| {
            let rules = get_rules(Formula::parse(s));
            match_rules(rules, vec![Rule::operation_expr]);
        });
    }
}
