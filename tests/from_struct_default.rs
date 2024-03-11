use structural_convert::StructuralConvert;

#[test]
fn variant_is_named_default_to_field() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        z: i8,
        x: u32,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from(Rhs))]
    struct Lhs {
        z: i32,
        x: u32,
        #[convert(from(default))]
        y: u8,
    }

    assert_eq!(
        Lhs {
            z: 1,
            x: 2,
            y: Default::default()
        },
        Rhs { z: 1, x: 2 }.into()
    );
}
