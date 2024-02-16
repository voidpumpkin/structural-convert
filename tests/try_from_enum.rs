use structural_convert::StructuralConvert;

#[test]
fn variant_is_unit() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A,
        B,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    enum Lhs {
        A,
        B,
    }

    assert_eq!(Lhs::A, Rhs::A.try_into().unwrap());
    assert_eq!(Lhs::B, Rhs::B.try_into().unwrap());
}

#[test]
fn variant_is_unnamed() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A(i8, u32),
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    enum Lhs {
        A(i32, u32),
    }

    assert_eq!(Lhs::A(1, 2), Rhs::A(1, 2).try_into().unwrap());
}

#[test]
fn variant_is_named() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A { z: i8, x: u32 },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    enum Lhs {
        A { z: i32, x: u32 },
    }

    assert_eq!(
        Lhs::A { z: 1, x: 2 },
        Rhs::A { z: 1, x: 2 }.try_into().unwrap()
    );
}
