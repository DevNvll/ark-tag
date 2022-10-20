extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::{collections::HashMap, fs, vec};

use pest::Parser;

#[derive(Parser)]
#[grammar = "rules.pest"]
pub struct ArkParser;

fn process(input: &HashMap<String, String>, rules: &str) -> Result<Vec<String>, ()> {
    let file = ArkParser::parse(Rule::rules, &rules)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    let mut added_tags = vec![];
    for expr in file.into_inner() {
        match expr.as_rule() {
            Rule::EOI => {
                break;
            }
            _ => {
                let mut key = String::new();
                let mut value = vec![];
                let mut condition = Rule::ANY_OF;
                let mut tags: Vec<String> = vec![];
                for pair in expr.into_inner() {
                    match pair.as_rule() {
                        Rule::property => {
                            key = pair.as_str().to_string();
                        }
                        Rule::value => {
                            value.push(pair.as_str().to_string().replace("\"", ""));
                        }
                        Rule::conditional => match pair.as_str() {
                            "ANY OF" => condition = Rule::ANY_OF,
                            "ALL OF" => condition = Rule::ALL_OF,
                            _ => (),
                        },
                        Rule::tag => {
                            tags.push(pair.as_str().to_string().replace("\"", ""));
                        }
                        _ => unreachable!(),
                    }
                }

                match condition {
                    Rule::ANY_OF => {
                        if value.iter().any(|v| input.get(&key).unwrap().contains(v)) {
                            added_tags = tags;
                        }
                    }
                    Rule::ALL_OF => {
                        if value.iter().all(|v| input.get(&key).unwrap().contains(v)) {
                            added_tags = tags;
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }
    }

    Ok(added_tags)
}

fn main() {
    let rules_file = fs::read_to_string("rules.ark").expect("cannot read file");

    let mut input = HashMap::new();

    input.insert("title".to_string(), "go overview".to_string());
    input.insert("body".to_string(), "test body tutorial rust".to_string());

    let result = process(&input, &rules_file);

    println!("result: {:?}", result);
}
