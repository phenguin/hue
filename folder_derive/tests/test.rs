#![feature(conservative_impl_trait)]
#![feature(specialization)]
#![feature(optin_builtin_traits)]
#![feature(proc_macro)]
#![allow(unused_imports)]
extern crate folder_derive;

use folder_derive::foldable_types;
use std::marker::PhantomData;
use std::iter::{self,IntoIterator,Iterator};

// foldable_types!{
//     foldtest,
//     #[derive(Clone, Debug)]
//     pub struct TestStruct(pub usize, pub TestStructTwo);
//     #[derive(Clone, Debug)]
//     pub struct TestStructTwo {
//         pub a: i32
//     }
//     #[derive(Clone, Debug)]
//     pub struct TestStructThree();

//     #[derive(Clone, Debug)]
//     pub enum TestEnum {
//         A(i32),
//         B(::tests::foldtest::TestEnumTwo, self::TestStruct)
//     }

//     #[derive(Clone, Debug)]
//     pub enum TestEnumTwo {
//         C(bool,i32),
//         D(usize),
//         E,
//     }
// }


foldable_types!{test_inc,
                // #[derive(Eq, PartialEq)]
                pub struct S1{pub prim: u32, pub sub: S2}
                // #[derive(Eq, PartialEq)]
                pub struct S2(pub u32);
}

#[test]
fn test_inc() {
    use self::test_inc::Folder;
    use self::test_inc::*;

    struct IncFolder;
    impl self::test_inc::Folder for IncFolder {
        fn mut_map_S2(&mut self, it: &mut S2) {
            it.0 += 1;
        }
    }

    let x = S1 {
        prim: 10,
        sub: S2(0),
    };
    assert_eq!(x.sub.0 + 1, IncFolder.fold_S1(x).sub.0);
}

#[test]
fn test_tmp() {
    // let mut x = Box::new(5);
    // let _ = (&mut x).into_iter();

    struct MutIterWrapper<T, U> {inner:T, _item: PhantomData<U>};
    #[derive(Debug)]
    struct UnitWrapper<T>{inner:T};

    impl<T, U> MutIterWrapper<T, U> {
        fn new(x: T) -> Self {
            Self {
                inner: x,
                _item:PhantomData,
            }
        }
    }

    use std::ops::{Deref, DerefMut};
    impl<'a, U: 'a, T: 'a + Deref<Target=U> + DerefMut> IntoIterator for &'a mut UnitWrapper<T> {
        type Item = &'a mut U;
        type IntoIter = iter::Once<&'a mut U>;
        fn into_iter(self) -> Self::IntoIter {
            iter::once((&mut self.inner).deref_mut())
        }
    }

    impl<'a, U, T> IntoIterator for &'a mut MutIterWrapper<T, U>
    where &'a mut T: IntoIterator<Item=&'a mut U>{
        type Item = &'a mut U;
        type IntoIter = <&'a mut T as IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            (&mut self.inner).into_iter()
        }
    }

    fn test() {
    let mut x = UnitWrapper::<Box<u32>> {
        inner: Box::new(5),
    };

    println!("{:?}", x);
    (&mut x).into_iter().for_each(|x| *x += 10);
    println!("{:?}", x);

    assert_eq!(0, 1);
    }
    test();

}
