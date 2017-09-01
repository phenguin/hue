use errors::*;
use std::fmt;
use pest::Parser;
use pest::iterators::Pair;
use pest::inputs::StringInput;
use std::convert::TryFrom;



#[derive(Debug, Clone, PartialEq)]
enum LispLit {
    I(i64),
    F(f64),
    S(String),
    B(bool),
}

impl fmt::Display for LispLit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::LispLit::*;
        match self {
            &I(x) => write!(f, "{}", x),
            &F(x) => write!(f, "{}", x),
            &S(ref x) => write!(f, "{}", x),
            &B(x) => write!(f, "{}", x),
        }
    }
}

const _GRAMMAR: &'static str = include_str!("./lisp.pest"); 

#[derive(Parser)]
#[grammar = "parsers/lisp.pest"]
pub struct LispParser;

type Name = String;

impl TryFrom<Pair<Rule, StringInput>> for LispLit {
    type Error = Error;
    fn try_from(pair: Pair<Rule, StringInput>) -> Res<LispLit> {
        use self::LispLit::*;
        let rule = pair.as_rule();
        match rule {
            Rule::literal => {
                let p = pair.into_inner().next().expect("No body for literal.");
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
                    _ => bail!("Line {} -- Unexpected: ({:#?}){:#?}", line!(), rule, span),
                }
            },
            _ => bail!("Line {} -- Unexpected: {:#?}", line!(), pair),
        }

    }
}

impl TryFrom<Pair<Rule, StringInput>> for LispExpr {
    type Error = Error;
    fn try_from(pair: Pair<Rule, StringInput>) -> Res<LispExpr> {
        use self::LispExpr::*;
        let rule = pair.as_rule();
        match rule {
            Rule::expr => {
                let p = pair.into_inner().next().expect("Expr has no body");
                let rule = p.as_rule();
                match rule {
                    Rule::ident => Ok(Ident(p.into_span().as_str().to_owned())),
                    Rule::sexp => LispSexp::try_from(p).map(Sexp),
                    Rule::literal => LispLit::try_from(p).map(Lit),
                    _ => bail!("Line {} -- Unexpected: {:#?}", line!(), p),
                }
            },
            _ => bail!("Line {} -- Unexpected: {:#?}", line!(), pair),
        }
    }
}

impl TryFrom<Pair<Rule, StringInput>> for LispSexp {
    type Error = Error;
    fn try_from(pair: Pair<Rule, StringInput>) -> Res<LispSexp> {
        use self::LispExpr::*;
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
                        _ => bail!("Line {} -- Unexpected: {:#?}", line!(), p),
                    }
                }
                Ok(LispSexp{ contents: res })
            },
            _ => bail!("Line {} -- Unexpected: {:#?}", line!(), pair),
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
