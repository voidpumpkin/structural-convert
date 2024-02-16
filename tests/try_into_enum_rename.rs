use structural_convert::StructuralConvert;

#[test]
fn variant_is_unit_non_targeted() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_into = "Lhs")]
    enum Rhs {
        #[convert(try_into(rename = "X"))]
        A,
        B,
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        X,
        B,
    }

    assert_eq!(Lhs::X, Rhs::A.try_into().unwrap());
    assert_eq!(Lhs::B, Rhs::B.try_into().unwrap());
}

#[test]
fn variant_is_unit_targeted() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_into = "Lhs1", try_into = "Lhs2")]
    enum Rhs {
        #[convert(try_into(for = "Lhs1", rename = "X"), try_into(for = "Lhs2"))]
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

    assert_eq!(Lhs1::X, Rhs::A.try_into().unwrap());
    assert_eq!(Lhs1::B, Rhs::B.try_into().unwrap());

    assert_eq!(Lhs2::A, Rhs::A.try_into().unwrap());
    assert_eq!(Lhs2::B, Rhs::B.try_into().unwrap());
}

#[test]
fn variant_is_unnamed() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_into = "Lhs")]
    enum Rhs {
        #[convert(try_into(rename = "X"))]
        A(i8, u32),
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        X(i32, u32),
    }

    assert_eq!(Lhs::X(1, 2), Rhs::A(1, 2).try_into().unwrap());
}

#[test]
fn variant_is_named() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_into = "Lhs")]
    enum Rhs {
        #[convert(try_into(rename = "X"))]
        A { z: i8, x: u32 },
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        X { z: i32, x: u32 },
    }

    assert_eq!(
        Lhs::X { z: 1, x: 2 },
        Rhs::A { z: 1, x: 2 }.try_into().unwrap()
    );
}
