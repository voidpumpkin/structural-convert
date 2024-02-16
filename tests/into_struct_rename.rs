use structural_convert::StructuralConvert;

#[test]
fn fields_named_not_targeted() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(path = "Lhs"))]
    struct Rhs {
        #[convert(into(rename = "z"))]
        a: i8,
        x: u32,
    }

    #[derive(Debug, PartialEq)]
    struct Lhs {
        z: i8,
        x: u32,
    }

    assert_eq!(Lhs { z: 1, x: 2 }, Rhs { a: 1, x: 2 }.into());
}

#[test]
fn fields_named_targeted() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(path = "Lhs1"), into(path = "Lhs2"))]
    struct Rhs {
        #[convert(into(for = "Lhs1", rename = "z"))]
        a: i8,
        x: u32,
    }

    #[derive(Debug, PartialEq)]
    struct Lhs1 {
        z: i8,
        x: u32,
    }

    #[derive(Debug, PartialEq)]
    struct Lhs2 {
        a: i8,
        x: u32,
    }

    assert_eq!(Lhs1 { z: 1, x: 2 }, Rhs { a: 1, x: 2 }.into());
    assert_eq!(Lhs2 { a: 1, x: 2 }, Rhs { a: 1, x: 2 }.into());
}
