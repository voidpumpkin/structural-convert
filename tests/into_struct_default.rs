use structural_convert::StructuralConvert;

#[test]
fn variant_is_named_default_to_field() {
    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(into(path = "Lhs", default_for_fields("y", "e")))]
    struct Rhs {
        z: i8,
        x: u32,
    }

    #[derive(Debug, PartialEq)]
    struct Lhs {
        z: i32,
        x: u32,
        y: u32,
        e: u32,
    }

    assert_eq!(
        Lhs {
            z: 1,
            x: 2,
            y: Default::default(),
            e: Default::default()
        },
        Rhs { z: 1, x: 2 }.into()
    );
}
