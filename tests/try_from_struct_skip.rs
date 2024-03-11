use structural_convert::StructuralConvert;

#[test]
fn fields_unnamed() {
    #[derive(Debug, PartialEq)]
    struct Rhs(i8, u32, u8);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(Rhs))]
    struct Lhs(i32, u32);

    assert_eq!(Lhs(1, 2), Rhs(1, 2, 3).try_into().unwrap());
    assert_eq!(Lhs(1, 2), Rhs(1, 2, 3).try_into().unwrap());
}

#[test]
fn fields_named() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        z: i8,
        x: u32,
        y: u8,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(Rhs))]
    struct Lhs {
        z: i32,
        x: u32,
    }

    assert_eq!(
        Lhs { z: 1, x: 2 },
        Rhs { z: 1, x: 2, y: 3 }.try_into().unwrap()
    );
    assert_eq!(
        Lhs { z: 1, x: 2 },
        Rhs { z: 1, x: 2, y: 3 }.try_into().unwrap()
    );
}
