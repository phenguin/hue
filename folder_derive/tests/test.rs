#![allow(unused_imports)]
extern crate folder_derive;

mod tests {
    use folder_derive::*;
    foldable_types!{
        foldtest,
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
            B(::tests::foldtest::TestEnumTwo, self::TestStruct)
        }

        #[derive(Clone, Debug)]
        pub enum TestEnumTwo {
            C(bool,i32),
            D(usize),
            E,
        }
    }
    

    foldable_types!{test_inc,
                    #[derive(Eq, PartialEq)]
                    pub struct S1{pub prim: u32, pub sub: S2}
                    #[derive(Eq, PartialEq)]
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

}
