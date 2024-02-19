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

    impl TryFrom<i32> for RhsEnum {
        type Error = &'static str;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            Ok(match value {
                0 => RhsEnum::Mobile,
                1 => RhsEnum::Home,
                _ => return Err(""),
            })
        }
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    pub struct Lhs {
        #[convert(try_from(as = "RhsEnum"))]
        pub r#type: LhsEnum,
    }

    #[derive(Debug, PartialEq, Eq, StructuralConvert, Default)]
    #[convert(try_from(path = "RhsEnum"))]
    pub enum LhsEnum {
        #[default]
        Mobile,
        Home,
    }

    assert_eq!(
        Lhs {
            r#type: LhsEnum::Home
        },
        Rhs { r#type: 1 }.try_into().unwrap()
    );
}

