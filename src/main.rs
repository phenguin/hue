extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate dump;

use pest::Parser;

#[derive(Parser)]
#[grammar = "lisp.pest"]
struct LispParser;

fn main() {
    let pairs = LispParser::parse_str(Rule::program, "(a b c)").unwrap_or_else(|e| panic!("{}", e));

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
