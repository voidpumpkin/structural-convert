use structural_convert::StructuralConvert;

#[test]
fn variant_is_unit_non_targeted() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        X,
        B,
        C,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from = "Rhs")]
    enum Lhs {
        #[convert(try_from(rename = "X"))]
        A,
        B,
        C,
    }

    assert_eq!(Lhs::A, Rhs::X.try_into().unwrap());
    assert_eq!(Lhs::B, Rhs::B.try_into().unwrap());
    assert_eq!(Lhs::C, Rhs::C.try_into().unwrap());
}

#[test]
fn variant_is_unit_targeted() {
    #[derive(Debug, PartialEq)]
    enum Rhs1 {
        X,
        B,
        C,
    }

    #[derive(Debug, PartialEq)]
    enum Rhs2 {
        A,
        B,
        C,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from = "Rhs1", try_from = "Rhs2")]
    enum Lhs {
        #[convert(try_from(for = "Rhs1", rename = "X"), try_from(for = "Rhs2"))]
        A,
        B,
        C,
    }

    assert_eq!(Lhs::A, Rhs1::X.try_into().unwrap());
    assert_eq!(Lhs::B, Rhs1::B.try_into().unwrap());
    assert_eq!(Lhs::C, Rhs1::C.try_into().unwrap());

    assert_eq!(Lhs::A, Rhs2::A.try_into().unwrap());
    assert_eq!(Lhs::B, Rhs2::B.try_into().unwrap());
    assert_eq!(Lhs::C, Rhs2::C.try_into().unwrap());
}

#[test]
fn variant_is_unnamed() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        X(i8, u32, u8),
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from = "Rhs")]
    enum Lhs {
        #[convert(try_from(rename = "X"))]
        A(i32, u32),
    }

    assert_eq!(Lhs::A(1, 2), Rhs::X(1, 2, 3).try_into().unwrap());
}

#[test]
fn variant_is_named() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        X { z: i8, x: u32, y: u8 },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from = "Rhs")]
    enum Lhs {
        #[convert(try_from(rename = "X"))]
        A { z: i32, x: u32 },
    }

    assert_eq!(
        Lhs::A { z: 1, x: 2 },
        Rhs::X { z: 1, x: 2, y: 3 }.try_into().unwrap()
    );
}
