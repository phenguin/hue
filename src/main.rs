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
use folder_derive::foldable_types;
use parsers::Parseable;
use util::*;

#[macro_use] mod util;
mod errors;
mod multitree;
mod parsers;

foldable_types!{foldtest,
    #[derive(Clone, Debug)]
    pub struct TestStruct(pub usize, pub TestStructTwo);
    #[derive(Clone, Debug)]
    pub struct TestStructTwo {
        pub a: i32
    }
    #[derive(Clone, Debug)]
    pub struct TestStructThree();

    #[derive(Clone, Debug)]
    pub enum TestEnum {
        A(i32),
        B(::foldtest::TestEnumTwo, self::TestStruct)
    }

    #[derive(Clone, Debug)]
    pub enum TestEnumTwo {
        C(bool,i32),
        D(usize),
        E,
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
    // use parsers::lisp::{LispProgram};
    // let it = "(f (h 1 2) (g 3 4 5))";
    // dump!(it.parsed::<LispProgram>());
    use foldtest::*;
    let mut s1 = TestStruct(20, TestStructTwo{a: 3});
    let mut s2 = TestStructTwo{a:4};

    let mut e1_1 = TestEnum::A(5);
    let mut e2 = TestEnumTwo::C(true, 2);
    let mut e1_2 = TestEnum::B(e2.clone(), s1.clone());


    // dump!(s1);
    // DefaultFolder.fold_teststruct(&mut s1);
    // DefaultFolder.fold_teststructtwo(&mut s2);
    // dump!(s1);
    struct IncFolder;
    impl foldtest::Folder for IncFolder {
        fn mut_fold_TestStructTwo(&mut self, it: &mut TestStructTwo) {
            it.a += 1;
        }
        fn mut_map_TestStruct(&mut self, it: &mut TestStruct) {
            it.0 += 10;
        }
        fn mut_fold_TestEnumTwo(&mut self, it: &mut TestEnumTwo) {
            if let TestEnumTwo::C(_, ref mut n) = *it {
                *n += 100
            }
        }
    }

    // Start state
    // [src/main.rs:105] e1_2: TestEnum = B(
    //     C(
    //         true,
    //         2
    //     ),
    //     TestStruct(
    //         20,
    //         TestStructTwo {
    //             a: 3
    //         }
    //     )
    // ); 


    dump!(e1_2);
    dump!(e1_1.clone());

    fn f(x: Box<u32>) -> Box<u32> {
        x
    }
    dump!(f(Box::new(5)));

    let new = IncFolder.fold_TestEnum(IncFolder.fold_TestEnum(e1_2));
    dump!(new);

    // IncFolder.fold_teststruct(&mut s1);
    // dump!(s1);

    Ok(())
}
