#![recursion_limit = "1024"]
#![feature(try_from)]
#![feature(core_intrinsics)]

mod parsers;
mod errors;

pub fn get_type_of<T>(_: &T) -> String {
	  unsafe {
		    std::intrinsics::type_name::<T>()
	  }.to_owned()
}

#[macro_export]
macro_rules! dump(
	  ($($a:expr),*) => {
		    println!(concat!("[", file!(), ":", line!(), "] ", $(stringify!($a), ": {} = {:#?}; "),*), $($crate::get_type_of(&$a), $a),*);
	  }
);

extern crate pest;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate pest_derive;

use errors::*;

use std::fmt;
use pest::Parser;
use pest::iterators::Pair;
use pest::inputs::StringInput;
use std::convert::TryFrom;


fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:#?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run2() -> Res<()> {
    use parsers::tree::TreeParser;
    Ok(())
    
}

fn run() -> Res<()> {
    use parsers::lisp;
    let pairs = lisp::LispParser::parse_str(lisp::Rule::sexp, "(f (h 1 2) (g 3 4 5))").expect("Pest parsing failed.");
    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:#?}", pair.as_rule());
        println!("Span:    {:#?}", pair.clone().into_span());
        println!("Text:    {}", pair.clone().into_span().as_str());
    }
    Ok(())
}
