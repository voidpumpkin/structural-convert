use structural_convert::StructuralConvert;

#[test]
fn fields_named_i32() {
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
    #[convert(try_into(Rhs))]
    pub struct Lhs {
        #[convert(try_into(as = "RhsEnum"))]
        pub r#type: LhsEnum,
    }

    #[derive(Debug, PartialEq, Eq, StructuralConvert)]
    #[convert(try_into(RhsEnum))]
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
        .try_into()
        .unwrap(),
    );
}

#[test]
fn fields_named_option() {
    #[derive(Debug, PartialEq)]
    struct Q(u32);
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Q))]
    struct W(u32);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_into(Lhs))]
    struct Rhs {
        z: i8,
        #[convert(try_into(Lhs, as = "Option::<Q>"))]
        x: Q,
    }

    #[derive(Debug, PartialEq)]
    struct Lhs {
        z: i32,
        x: Option<W>,
    }

    assert_eq!(
        Lhs {
            z: 1,
            x: Some(W(2))
        },
        Rhs { z: 1, x: Q(2) }.try_into().unwrap()
    );
}
