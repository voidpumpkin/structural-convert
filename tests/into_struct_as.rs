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
        Work = 2,
    }

    impl From<RhsEnum> for i32 {
        fn from(value: RhsEnum) -> Self {
            value as i32
        }
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(path = "Rhs"))]
    pub struct Lhs {
        #[convert(into(as = "RhsEnum"))]
        pub r#type: LhsEnum,
    }

    #[derive(Debug, PartialEq, Eq, StructuralConvert)]
    #[convert(into(path = "RhsEnum"))]
    pub enum LhsEnum {
        Mobile,
        Home,
        Work,
    }

    assert_eq!(
        Rhs { r#type: 1 },
        Lhs {
            r#type: LhsEnum::Home
        }
        .into(),
    );
}
