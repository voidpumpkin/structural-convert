use structural_convert::StructuralConvert;

#[test]
fn variant_is_unit_default() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(Lhs, default))]
    enum Rhs {
        #[convert(into(skip))]
        A,
    }

    #[derive(Debug, PartialEq, Default)]
    enum Lhs {
        #[default]
        X,
    }

    assert_eq!(Lhs::default(), Rhs::A.into());
}

#[test]
fn variant_is_unnamed_default() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(Lhs, default))]
    enum Rhs {
        #[convert(into(skip))]
        A(i8, u32),
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        X(i32, u32),
    }

    impl Default for Lhs {
        fn default() -> Self {
            Lhs::X(Default::default(), Default::default())
        }
    }

    assert_eq!(Lhs::default(), Rhs::A(1, 2).into());
}

#[test]
fn variant_is_named_default() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(Lhs, default))]
    enum Rhs {
        #[convert(into(skip))]
        A { z: i8, x: u32 },
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        X {},
    }

    impl Default for Lhs {
        fn default() -> Self {
            Lhs::X {}
        }
    }

    assert_eq!(Lhs::default(), Rhs::A { z: 1, x: 2 }.into());
}

#[test]
fn variant_is_named_default_to_field() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(Lhs, default_for_fields("y", "e")))]
    enum Rhs {
        A { z: i8, x: u32 },
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        A { z: i32, x: u32, y: u32, e: u32 },
    }

    assert_eq!(
        Lhs::A {
            z: 1,
            x: 2,
            y: Default::default(),
            e: Default::default()
        },
        Rhs::A { z: 1, x: 2 }.into()
    );
}
