#![feature(proc_macro)]
#![feature(log_syntax)]
#![recursion_limit = "1024"]
#![feature(try_from)]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![feature(trace_macros)]


#[macro_use] extern crate error_chain;
#[macro_use] extern crate pest_derive;
extern crate folder_derive;
extern crate pest;
use errors::*;
use folder_derive::foldable;
use parsers::Parseable;
use util::*;

#[macro_use] mod util;
mod errors;
mod multitree;
mod parsers;

foldable!{Testing,
    enum Test {
        A(i32),
        B(TestTwo, TestTwo)
    }

    enum TestTwo {
        C(bool,i32),
        D(usize),
        E()
    }
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
            println!("backtrace: {:#?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

#[allow(dead_code)]
fn run2() -> Res<()> {
    Ok(())
}


fn run() -> Res<()> {
    use parsers::lisp::{LispProgram};
    let it = "(f (h 1 2) (g 3 4 5))";
    dump!(it.parsed::<LispProgram>());

    let x = Test::A(5);
    let y = TestTwo::C(true, 2);
    DefaultFolder.fold_test(x);
    DefaultFolder.fold_testtwo(y);

    Ok(())
}
