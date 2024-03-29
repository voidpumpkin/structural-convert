use structural_convert::StructuralConvert;

#[derive(Debug, PartialEq)]
struct Q(u32);
#[derive(Debug, PartialEq, StructuralConvert)]
#[convert(from(Q))]
struct W(u32);

#[test]
fn fields_unnamed() {
    #[derive(Debug, PartialEq)]
    struct Rhs(i8, Option<Q>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    struct Lhs(i32, Option<W>);

    assert_eq!(Lhs(1, Some(W(2))), Rhs(1, Some(Q(2))).into());
}

#[test]
fn fields_named() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        z: i8,
        x: Option<Q>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    struct Lhs {
        z: i32,
        x: Option<W>,
    }

    assert_eq!(
        Lhs {
            z: 1,
            x: Some(W(2))
        },
        Rhs {
            z: 1,
            x: Some(Q(2))
        }
        .into()
    );
}
