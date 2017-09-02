#![feature(associated_constants)]

pub mod lisp;
pub mod tree;
use pest::{RuleType, Parser};
use pest::iterators::Pair;
use pest::inputs::{StringInput, Input};
use std::str::FromStr;
use std::marker::Sized;

use errors::*;

trait RepParser {
    type Rule: RuleType;
    type Rep;
    const rule: Self::Rule;
    fn represent(Pair<Self::Rule, StringInput>) -> Res<Self::Rep>;
}

trait Parseable {
    type Err;
    type Input: Input;
    fn parsed<T>(&self) -> Result<T, Self::Err>
        where T: FromParse;

}

impl<T: AsRef<str>> Parseable for T {
    type Input = StringInput;
    type Err = Error;
    fn parsed<U: FromParse>(&self) -> Res<U> {
        let s = self.as_ref();
        let pairs = U::Parser::parse_str(U::rule, s);
        match pairs {
            Err(_) => return Err("Parsing failed.".into()),
            Ok(mut pairs) => {
                match pairs.next() {
                    Some(data) => {
                        U::represent(data)
                    },
                    None => Err("Nothing parsed.".into()),
                }
            }
        }

    }
}

trait FromParse: Sized {
    type Rule: RuleType;
    type Parser: Parser<Self::Rule>;
    const rule: Self::Rule;
    fn represent<I:Input>(Pair<Self::Rule, I>) -> Res<Self>;
}
