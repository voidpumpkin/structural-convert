/// Case not implemented
/// ```
/// #[derive(Debug, PartialEq, StructuralConvert)]
/// #[convert(into(for = "Lhs", skip_after = 2))]
/// struct Rhs(i8, u32, u8);
///
/// #[derive(Debug, PartialEq)]
/// struct Lhs(i32, u32);
///
/// assert_eq!(Lhs(1, 2), Rhs(1, 2, 3).into());
/// ```
#[test]
fn fields_unnamed() {}

/// Case not implemented
/// Idea for api 1:
/// ```
/// #[derive(Debug, PartialEq, StructuralConvert)]
/// #[convert(into(for = "Lhs",  skip("y", "e")))]
/// struct Rhs {
///     z: i8,
///     x: u32,
///     y: u8,
///     e: u8,
/// }
///
/// #[derive(Debug, PartialEq)]
/// struct Lhs {
///     z: i32,
///     x: u32,
/// }
///
/// assert_eq!(Lhs { z: 1, x: 2 }, Rhs { z: 1, x: 2, y: 3, e: 4 }.into());
/// ```
///
/// Idea for api 2:
/// ```
/// #[derive(Debug, PartialEq, StructuralConvert)]
/// #[convert(into = "Lhs")]
/// struct Rhs {
///     z: i8,
///     x: u32,
///     #[convert(into(for = "Lhs", skip))]
///     y: u8,
/// }
///
/// #[derive(Debug, PartialEq)]
/// struct Lhs {
///     z: i32,
///     x: u32,
/// }
///
/// assert_eq!(Lhs { z: 1, x: 2 }, Rhs { z: 1, x: 2, y: 3 }.into());
/// ```
#[test]
fn fields_named() {}
