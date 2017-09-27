use errors::*;

use pest::iterators::Pair;
use pest::inputs::StringInput;

use std::convert::TryFrom;

const _GRAMMAR: &'static str = include_str!("./tree.pest");

#[derive(Parser)]
#[grammar = "parsers/tree.pest"]
#[allow(dead_code)]
pub struct TreeParser;

impl TryFrom<Pair<Rule, StringInput>> for Tree {
    type Error = Error;
    fn try_from(p: Pair<Rule, StringInput>) -> Res<Tree> {
        let rule = p.as_rule();
        match rule {
            Rule::int => {
                p.into_span()
                    .as_str()
                    .parse()
                    .map(|n| {
                        Tree {
                            key: n,
                            children: Vec::new(),
                        }
                    })
                    .chain_err(|| "bad parse")
            }
            Rule::tree => unreachable!(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree {
    key: i64,
    children: Vec<Tree>,
}
