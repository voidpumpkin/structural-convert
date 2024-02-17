use structural_convert::StructuralConvert;

#[test]
fn variant_is_unit_default_to_field() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        X,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(path = "Rhs"))]
    enum Lhs {
        #[convert(from(default))]
        A,
    }

    assert_eq!(Lhs::A, Rhs::X.into());
}

#[test]
fn variant_is_unnamed_default_to_field() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        X(i8, u32, u8),
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(path = "Rhs"))]
    enum Lhs {
        #[convert(from(default))]
        A(i32, u32),
    }

    assert_eq!(
        Lhs::A(Default::default(), Default::default()),
        Rhs::X(1, 2, 3).into()
    );
}

#[test]
fn variant_is_named_default_to_field() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A { z: i8, x: u32 },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(path = "Rhs"))]
    enum Lhs {
        A {
            z: i32,
            x: u32,
            #[convert(from(default))]
            y: u8,
        },
    }

    assert_eq!(
        Lhs::A {
            z: 1,
            x: 2,
            y: Default::default()
        },
        Rhs::A { z: 1, x: 2 }.into()
    );
}
