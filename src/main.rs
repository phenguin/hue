extern crate pest;


#[macro_use]
extern crate either;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate dump;

use pest::Parser;
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

#[derive(Debug, Clone)]
struct LispSexp {
    contents: Option<(Either<Name, Box<LispSexp>>, Vec<LispExpr>)>
}

#[derive(Debug, Clone)]
enum LispExpr {
    Ident(Name),
    Sexp(LispSexp),
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
