use structural_convert::StructuralConvert;

#[test]
fn unit() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(Lhs))]
    struct Rhs;

    #[derive(Debug, PartialEq)]
    struct Lhs;

    assert_eq!(Lhs, Rhs.into());
}

#[test]
fn fields_unnamed() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(Lhs))]
    struct Rhs(i8, u32);

    #[derive(Debug, PartialEq)]
    struct Lhs(i32, u32);

    assert_eq!(Lhs(1, 2), Rhs(1, 2).into());
}

#[test]
fn fields_named() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(Lhs))]
    struct Rhs {
        z: i8,
        x: u32,
    }

    #[derive(Debug, PartialEq)]
    struct Lhs {
        z: i32,
        x: u32,
    }

    assert_eq!(Lhs { z: 1, x: 2 }, Rhs { z: 1, x: 2 }.into());
}
