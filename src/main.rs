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
    use parsers::lisp::LispProgram;
    let it =  "(f (h 1 2) (g 3 4 5))";
    dump!(it.parse::<LispProgram>());
    Ok(())
}
