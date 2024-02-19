use structural_convert::StructuralConvert;

#[derive(Debug, PartialEq)]
struct Q(u32);
#[derive(Debug, PartialEq, StructuralConvert)]
#[convert(try_from(path = "Q"))]
struct W(u32);

#[test]
fn fields_unnamed() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_into(path = "Lhs"))]
    struct Rhs(i8, Option<Q>);

    #[derive(Debug, PartialEq)]
    struct Lhs(i32, Option<W>);

    assert_eq!(Lhs(1, Some(W(2))), Rhs(1, Some(Q(2))).try_into().unwrap());
}

#[test]
fn fields_named() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_into(path = "Lhs"))]
    struct Rhs {
        z: i8,
        x: Option<Q>,
    }

    #[derive(Debug, PartialEq)]
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
        .try_into()
        .unwrap()
    );
}
