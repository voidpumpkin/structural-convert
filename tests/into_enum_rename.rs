use structural_convert::StructuralConvert;

#[test]
fn variant_is_unit_non_targeted() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into = "Lhs")]
    enum Rhs {
        #[convert(into(rename = "X"))]
        A,
        B,
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        X,
        B,
    }

    assert_eq!(Lhs::X, Rhs::A.into());
    assert_eq!(Lhs::B, Rhs::B.into());
}

#[test]
fn variant_is_unit_targeted() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into = "Lhs1", into = "Lhs2")]
    enum Rhs {
        #[convert(into(for = "Lhs1", rename = "X"), into(for = "Lhs2"))]
        A,
        B,
    }

    #[derive(Debug, PartialEq)]
    enum Lhs1 {
        X,
        B,
    }

    #[derive(Debug, PartialEq)]
    enum Lhs2 {
        A,
        B,
    }

    assert_eq!(Lhs1::X, Rhs::A.into());
    assert_eq!(Lhs1::B, Rhs::B.into());

    assert_eq!(Lhs2::A, Rhs::A.into());
    assert_eq!(Lhs2::B, Rhs::B.into());
}

#[test]
fn variant_is_unnamed() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into = "Lhs")]
    enum Rhs {
        #[convert(into(rename = "X"))]
        A(i8, u32),
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        X(i32, u32),
    }

    assert_eq!(Lhs::X(1, 2), Rhs::A(1, 2).into());
}

#[test]
fn variant_is_named() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into = "Lhs")]
    enum Rhs {
        #[convert(into(rename = "X"))]
        A { z: i8, x: u32 },
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        X { z: i32, x: u32 },
    }

    assert_eq!(Lhs::X { z: 1, x: 2 }, Rhs::A { z: 1, x: 2 }.into());
}
