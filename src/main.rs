#![feature(try_from)]
extern crate pest;


#[macro_use]
extern crate either;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate dump;

use pest::Parser;
use pest::iterators::Pair;
use pest::inputs::StringInput;
use std::convert::TryFrom;
use either::Either;

const _GRAMMAR: &'static str = include_str!("./lisp.pest"); 

#[derive(Parser)]
#[grammar = "lisp.pest"]
struct LispParser;

#[derive(Debug, Clone, PartialEq)]
enum LispLit {
    I(i64),
    F(f64),
    S(String),
    B(bool),
}

type Name = String;

impl TryFrom<Pair<Rule, StringInput>> for LispLit {
    type Error = ();
    fn try_from(p: Pair<Rule, StringInput>) -> Result<LispLit, Self::Error> {
        use LispLit::*;
        let rule = p.as_rule();
        let span = p.into_span();
        match rule {
            Rule::float => span.as_str().parse().map(F).or_else(|_| Err(())),
            Rule::int => span.as_str().parse().map(I).or_else(|_| Err(())),
            Rule::string => Ok(S(span.as_str().to_owned())),
            Rule::boolean => span.as_str().parse().map(B).or_else(|_| Err(())),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone)]
enum LispExpr {
    Ident(Name),
    Sexp(Vec<LispExpr>),
    Lit(LispLit),
}

fn main() {
    let pairs = LispParser::parse_str(Rule::program, "(a \"hey there\" b c)").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());


        dump!(pair);

        // // A pair can be converted to an iterator of the tokens which make it up:
        // for inner_pair in pair.into_inner() {
        //     match inner_pair.as_rule() {
        //         Rule::ident => println!("Ident:  {:?}", inner_pair.into_span().as_str()),
        //         _ => unreachable!()
        //     };
        // }
    }
}
