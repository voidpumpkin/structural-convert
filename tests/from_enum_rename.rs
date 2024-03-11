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
    #[convert(from(Rhs))]
    enum Lhs {
        #[convert(from(rename = "X"))]
        A,
        B,
        C,
    }

    assert_eq!(Lhs::A, Rhs::X.into());
    assert_eq!(Lhs::B, Rhs::B.into());
    assert_eq!(Lhs::C, Rhs::C.into());
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
    #[convert(from(Rhs1), from(Rhs2))]
    enum Lhs {
        #[convert(from(Rhs1, rename = "X"), from(Rhs2))]
        A,
        B,
        C,
    }

    assert_eq!(Lhs::A, Rhs1::X.into());
    assert_eq!(Lhs::B, Rhs1::B.into());
    assert_eq!(Lhs::C, Rhs1::C.into());

    assert_eq!(Lhs::A, Rhs2::A.into());
    assert_eq!(Lhs::B, Rhs2::B.into());
    assert_eq!(Lhs::C, Rhs2::C.into());
}

#[test]
fn variant_is_unnamed() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        X(i8, u32, u8),
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    enum Lhs {
        #[convert(from(rename = "X"))]
        A(i32, u32),
    }

    assert_eq!(Lhs::A(1, 2), Rhs::X(1, 2, 3).into());
}

#[test]
fn variant_is_named() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        X { z: i8, x: u32, y: u8 },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    enum Lhs {
        #[convert(from(rename = "X"))]
        A { z: i32, x: u32 },
    }

    assert_eq!(Lhs::A { z: 1, x: 2 }, Rhs::X { z: 1, x: 2, y: 3 }.into());
}

#[test]
fn variant_is_named_fields_named_not_targeted() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A { z: i8, x: u32 },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    enum Lhs {
        A {
            #[convert(from(rename = "z"))]
            a: i32,
            x: u32,
        },
    }

    assert_eq!(Lhs::A { a: 1, x: 2 }, Rhs::A { z: 1, x: 2 }.into());
}

#[test]
fn variant_is_named_fields_named_targeted() {
    #[derive(Debug, PartialEq)]
    enum Rhs1 {
        A { z: i8, x: u32 },
    }

    #[derive(Debug, PartialEq)]
    enum Rhs2 {
        A { a: i8, x: u32 },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs1), from(Rhs2))]
    enum Lhs {
        A {
            #[convert(from(Rhs1::A, rename = "z"))]
            a: i32,
            x: u32,
        },
    }

    assert_eq!(Lhs::A { a: 1, x: 2 }, Rhs1::A { z: 1, x: 2 }.into());
    assert_eq!(Lhs::A { a: 1, x: 2 }, Rhs2::A { a: 1, x: 2 }.into());
}
