use structural_convert::StructuralConvert;

#[derive(Debug, PartialEq)]
struct Q(u32);
#[derive(Debug, PartialEq, StructuralConvert)]
#[convert(from(path = "Q"))]
struct W(u32);

#[test]
fn variant_is_unnamed() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A(i8, Option<Q>),
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(path = "Rhs"))]
    enum Lhs {
        A(i32, Option<W>),
    }

    assert_eq!(Lhs::A(1, Some(W(2))), Rhs::A(1, Some(Q(2))).into());
}

#[test]
fn variant_is_named() {
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A { z: i8, x: Option<Q> },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(path = "Rhs"))]
    enum Lhs {
        A { z: i32, x: Option<W> },
    }

    assert_eq!(
        Lhs::A {
            z: 1,
            x: Some(W(2))
        },
        Rhs::A {
            z: 1,
            x: Some(Q(2))
        }
        .into()
    );
}
