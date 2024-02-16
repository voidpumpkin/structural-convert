use structural_convert::StructuralConvert;

#[test]
fn variant_is_unit_non_targeted() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A,
        B,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    enum Lhs {
        #[convert(try_from(skip))]
        _C,
        A,
        B,
    }

    assert_eq!(Lhs::A, Rhs::A.try_into().unwrap());
    assert_eq!(Lhs::B, Rhs::B.try_into().unwrap());
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
    #[convert(try_from(path = "Rhs1"), try_from(path = "Rhs2"))]
    enum Lhs {
        #[convert(try_from(for = "Rhs1", skip), try_from(for = "Rhs2"))]
        C,
        A,
        B,
    }

    assert_eq!(Lhs::A, Rhs1::A.try_into().unwrap());
    assert_eq!(Lhs::B, Rhs1::B.try_into().unwrap());
    assert_eq!(Lhs::A, Rhs2::A.try_into().unwrap());
    assert_eq!(Lhs::B, Rhs2::B.try_into().unwrap());
    assert_eq!(Lhs::C, Rhs2::C.try_into().unwrap());
}

#[test]
fn variant_is_unnamed() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A(i8, u32, u8),
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    enum Lhs {
        A(i32, u32),
    }

    assert_eq!(Lhs::A(1, 2), Rhs::A(1, 2, 3).try_into().unwrap());
}

#[test]
fn variant_is_named() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A { z: i8, x: u32, y: u8 },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    enum Lhs {
        A { z: i32, x: u32 },
    }

    assert_eq!(
        Lhs::A { z: 1, x: 2 },
        Rhs::A { z: 1, x: 2, y: 3 }.try_into().unwrap()
    );
}
