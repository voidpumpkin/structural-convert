use structural_convert::StructuralConvert;

#[test]
fn variant_is_unit_non_targeted() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A,
        B,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    enum Lhs {
        #[convert(from(skip))]
        _C,
        A,
        B,
    }

    assert_eq!(Lhs::A, Rhs::A.into());
    assert_eq!(Lhs::B, Rhs::B.into());
}

#[test]
fn variant_is_unit_targeted() {
    #[derive(Debug, PartialEq)]
    enum Rhs1 {
        A,
        B,
    }

    #[derive(Debug, PartialEq)]
    enum Rhs2 {
        C,
        A,
        B,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs1), from(Rhs2))]
    enum Lhs {
        #[convert(from(Rhs1, skip), from(Rhs2))]
        C,
        A,
        B,
    }

    assert_eq!(Lhs::A, Rhs1::A.into());
    assert_eq!(Lhs::B, Rhs1::B.into());
    assert_eq!(Lhs::A, Rhs2::A.into());
    assert_eq!(Lhs::B, Rhs2::B.into());
    assert_eq!(Lhs::C, Rhs2::C.into());
}

#[test]
fn variant_is_unnamed() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A(i8, u32, u8),
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    enum Lhs {
        A(i32, u32),
    }

    assert_eq!(Lhs::A(1, 2), Rhs::A(1, 2, 3).into());
}

#[test]
fn variant_is_named() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A { z: i8, x: u32, y: u8 },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    enum Lhs {
        A { z: i32, x: u32 },
    }

    assert_eq!(Lhs::A { z: 1, x: 2 }, Rhs::A { z: 1, x: 2, y: 3 }.into());
}
