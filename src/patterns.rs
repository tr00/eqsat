use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;


#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PatternParser;

#[derive(Debug)]
struct Expr {
    children: Vec<Expr>,
}

#[derive(Debug)]
struct RewriteRule {

    // lhs expr
    // rhs expr
    //  - Q(x, y, z)
    //  - [ R_{kind}(id ; x1, x2, ..., xn) ]
}

#[derive(Debug)]
struct Relation {
    kind: String,
    vars: Vec<i32>,
}

fn to_query(pair: Pair<Rule>, vars: &mut HashMap<String, i32>) -> Relation {
    match pair.as_rule() {
        Rule::int => panic!("ints not implemented yet");
        Rule::expr => {
            let kind = pair.as_str().to_string();

            for child in pair.into_inner() {
                to_query(child, vars);
            }
        }
    }
    
}

pub fn parse(source: &str) {
    let res = PatternParser::parse(Rule::patlist, source);

    let pattern_list = res.expect("parse error!").next().unwrap();

    for pattern in pattern_list.into_inner() {
        match pattern.as_rule() {
            Rule::pattern => {
                // HANDLE PATTERN

                println!("PATTERN: {:?}", pattern);
                ()
            }
            Rule::EOI => (),
            _ => unreachable!()
        }

    }

}

