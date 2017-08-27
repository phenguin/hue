#![feature(try_from)]
extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate dump;

use pest::Parser;
use pest::iterators::Pair;
use pest::inputs::StringInput;
use std::convert::TryFrom;

macro_rules! oops {
    ($x:expr) => {
        {
            println!("OOPS!");
            dump!($x);
            unreachable!()
        }
    }
}

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

impl TryFrom<Pair<Rule, StringInput>> for LispExpr {
    type Error = ();
    fn try_from(pair: Pair<Rule, StringInput>) -> Result<LispExpr, Self::Error> {
        use LispExpr::*;
        let rule = pair.as_rule();
        match rule {
            Rule::expr => {
                let p = pair.into_inner().next().ok_or(())?;
                let rule = p.as_rule();
                match rule {
                    Rule::ident => Ok(Ident(p.into_span().as_str().to_owned())),
                    Rule::sexp => LispSexp::try_from(p).map(Sexp),
                    Rule::literal => LispLit::try_from(p).map(Lit),
                    _ => oops!(p),
                }
            },
            _ => oops!(pair),
        }
    }
}

impl TryFrom<Pair<Rule, StringInput>> for LispSexp {
    type Error = ();
    fn try_from(pair: Pair<Rule, StringInput>) -> Result<LispSexp, Self::Error> {
        use LispExpr::*;
        let rule = pair.as_rule();
        let mut res = Vec::new();
        match rule {
            Rule::sexp => {
                for p in pair.into_inner() {
                    let rule = p.as_rule();
                    match rule {
                        Rule::ident => res.push(Ident(p.into_span().as_str().to_owned())),
                        Rule::sexp => res.push(Sexp(LispSexp::try_from(p)?)),
                        Rule::expr => res.push((LispExpr::try_from(p)?)),
                        _ => oops!(p),
                    }
                }
                Ok(LispSexp{ contents: res })
            },
            _ => oops!(pair),
        }
    }
}

#[derive(Debug, Clone)]
struct LispSexp {
    contents: Vec<LispExpr>,
}

#[derive(Debug, Clone)]
enum LispExpr {
    Ident(Name),
    Sexp(LispSexp),
    Lit(LispLit),
}

fn main() {
    let pairs = LispParser::parse_str(Rule::sexp, "(f (h 1 2) (g 3 4 5))").unwrap_or_else(|e| panic!("{}", e));
    dump!(pairs);

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());

        dump!(pair);

        let it = LispSexp::try_from(pair).unwrap();

        dump!(it);
        // A pair can be converted to an iterator of the tokens which make it up:
        // for inner_pair in pair.into_inner() {
        //     match inner_pair.as_rule() {
        //         Rule::ident => println!("Ident:  {:?}", inner_pair.into_span().as_str()),
        //         _ => unreachable!()
        //     };
        // }
    }
}
