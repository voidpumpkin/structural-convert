use structural_convert::StructuralConvert;

#[test]
fn fields_named_not_targeted() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        z: i8,
        x: u32,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    struct Lhs {
        #[convert(from(rename = "z"))]
        a: i32,
        x: u32,
    }

    assert_eq!(Lhs { a: 1, x: 2 }, Rhs { z: 1, x: 2 }.into());
}

#[test]
fn fields_named_targeted() {
    #[derive(Debug, PartialEq)]
    struct Rhs1 {
        z: i8,
        x: u32,
    }

    #[derive(Debug, PartialEq)]
    struct Rhs2 {
        a: i8,
        x: u32,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs1), from(Rhs2))]
    struct Lhs {
        #[convert(from(Rhs1, rename = "z"))]
        a: i32,
        x: u32,
    }

    assert_eq!(Lhs { a: 1, x: 2 }, Rhs1 { z: 1, x: 2 }.into());
    assert_eq!(Lhs { a: 1, x: 2 }, Rhs2 { a: 1, x: 2 }.into());
}
