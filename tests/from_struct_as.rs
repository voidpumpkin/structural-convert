use structural_convert::StructuralConvert;

#[test]
fn fields_named() {
    #[derive(Debug, PartialEq)]
    pub struct Rhs {
        pub r#type: i32,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum RhsEnum {
        Mobile = 0,
        Home = 1,
    }

    impl From<i32> for RhsEnum {
        fn from(value: i32) -> Self {
            match value {
                0 => RhsEnum::Mobile,
                1 => RhsEnum::Home,
                _ => unimplemented!(),
            }
        }
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(path = "Rhs"))]
    pub struct Lhs {
        #[convert(from(as = "RhsEnum"))]
        pub r#type: LhsEnum,
    }

    #[derive(Debug, PartialEq, Eq, StructuralConvert, Default)]
    #[convert(from(path = "RhsEnum"))]
    pub enum LhsEnum {
        #[default]
        Mobile,
        Home,
    }

    assert_eq!(
        Lhs {
            r#type: LhsEnum::Home
        },
        Rhs { r#type: 1 }.into()
    );
}
