#![recursion_limit = "1024"]
#![feature(try_from)]
extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate dump;

#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Res;
        }
    }
}

use errors::*;

use pest::Parser;
use pest::iterators::Pair;
use pest::inputs::StringInput;
use std::convert::TryFrom;

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
    type Error = Error;
    fn try_from(p: Pair<Rule, StringInput>) -> Res<LispLit> {
        use LispLit::*;
        let rule = p.as_rule();
        let span = p.into_span();
        match rule {
            Rule::float => span.as_str().parse().map(F).chain_err(
                || format!("Bad parse float parse: {}", span.as_str())),
            Rule::int => span.as_str().parse().map(I).chain_err(
                || format!("Bad parse float parse: {}", span.as_str())),
            Rule::string => Ok(S(span.as_str().to_owned())),
            Rule::boolean => span.as_str().parse().map(B).chain_err(
                || format!("Bad parse float parse: {}", span.as_str())),
            _ => bail!("Unexpected: ({:?}){:?}", rule, span),
        }
    }
}

impl TryFrom<Pair<Rule, StringInput>> for LispExpr {
    type Error = Error;
    fn try_from(pair: Pair<Rule, StringInput>) -> Res<LispExpr> {
        use LispExpr::*;
        let rule = pair.as_rule();
        match rule {
            Rule::expr => {
                let p = pair.into_inner().next().expect("Expr has no body");
                let rule = p.as_rule();
                match rule {
                    Rule::ident => Ok(Ident(p.into_span().as_str().to_owned())),
                    Rule::sexp => LispSexp::try_from(p).map(Sexp),
                    Rule::literal => LispLit::try_from(p).map(Lit),
                    _ => bail!("Unexpected: {:?}", p),
                }
            },
            _ => bail!("Unexpected: {:?}", pair),
        }
    }
}

impl TryFrom<Pair<Rule, StringInput>> for LispSexp {
    type Error = Error;
    fn try_from(pair: Pair<Rule, StringInput>) -> Res<LispSexp> {
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
                        _ => bail!("Unexpected: {:?}", p),
                    }
                }
                Ok(LispSexp{ contents: res })
            },
            _ => bail!("Unexpected: {:?}", pair),
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
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Res<()> {
    let pairs = LispParser::parse_str(Rule::sexp, "(f (h 1 2) (g 3 4 5))").expect("Pest parsing failed.");
    dump!(pairs);

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());

        dump!(pair);

        let it = LispSexp::try_from(pair)?;

        dump!(it);
        // A pair can be converted to an iterator of the tokens which make it up:
        // for inner_pair in pair.into_inner() {
        //     match inner_pair.as_rule() {
        //         Rule::ident => println!("Ident:  {:?}", inner_pair.into_span().as_str()),
        //         _ => unreachable!()
        //     };
        // }
    }
    Ok(())
}
