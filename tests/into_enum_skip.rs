use structural_convert::StructuralConvert;

#[test]
fn variant_is_unit_non_targeted() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(path = "Lhs"))]
    enum Rhs {
        A,
        B,
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        A,
        B,
        _C,
    }

    assert_eq!(Lhs::A, Rhs::A.into());
    assert_eq!(Lhs::B, Rhs::B.into());
}

#[test]
fn variant_is_unit_targeted() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(path = "Lhs"))]
    enum Rhs {
        A,
        B,
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        A,
        B,
        _C,
    }

    assert_eq!(Lhs::A, Rhs::A.into());
    assert_eq!(Lhs::B, Rhs::B.into());
}

#[test]
fn variant_is_unnamed() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(path = "Lhs"))]
    enum Rhs {
        #[convert(into(for = "Lhs", skip_after = 2))]
        A(i8, u32, u8),
    }

    #[derive(Debug, PartialEq)]
    enum Lhs {
        A(i32, u32),
    }

    assert_eq!(Lhs::A(1, 2), Rhs::A(1, 2, 3).into());
}

/// Case not implemented
/// ```
/// #[derive(Debug, PartialEq, StructuralConvert)]
/// #[convert(into(path = "Lhs"))]
/// enum Rhs {
///     #[convert(into(for = "Lhs", skip("y", "e")))]
///     A { z: i8, x: u32, y: u8, e: i8 },
/// }
///
/// #[derive(Debug, PartialEq)]
/// enum Lhs {
///     A { z: i32, x: u32 },
/// }
///
/// assert_eq!(
///     Lhs::A { z: 1, x: 2 },
///     Rhs::A { z: 1, x: 2, y: 3, e: 4 }.into()
/// );
/// ```
#[test]
fn variant_is_named() {}
