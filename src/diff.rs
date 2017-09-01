#![feature(try_from)]
use errors::*;

use pest::Parser;
use pest::iterators::Pair;
use pest::inputs::StringInput;

use std::convert::TryFrom;

#[derive(Parser)]
#[grammar = "tree.pest"]
pub struct TreeParser;

impl TryFrom<Pair<Rule, StringInput>> for Tree {
    type Error = Error;
    fn try_from(p: Pair<Rule, StringInput>) -> Res<Tree> {
        let rule = p.as_rule();
        match rule {
            Rule::int => p.into_span().as_str().parse().map(|n| Tree {
                key: n,
                children: Vec::new(),
            }).chain_err(|| "bad parse"),
            Rule::tree => {
                unreachable!()
            },
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree {
    key: i64,
    children: Vec<Tree>,
}

fn parse(s: &str) -> Res<Tree> {
    unreachable!()
}
