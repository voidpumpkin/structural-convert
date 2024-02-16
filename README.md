# structural-convert

Derive conversion traits when items are structurally similar.

Inspired by serde and struct-convert crates.

## Features

- One to one fields mapping derive for
  - From
  - Into
  - TryFrom
  - TryInto
- Inner fields type conversion using `.into()`/`.try_into()`
- Field skip functionality when deriving for From and TryFrom
- Rename enum variants

## Features Wishlist

- Rename fields
- Fallback to default per variant/field
- Skipping fields for Into / TryInto, for:
  - Unnamed Enum variants
  - Named Enum variants
  - Unnamed Struct fields
  - Named Struct fields

## Examples

Check the tests folder for more examples, but here is some samples:

### Struct

```rs
#[derive(Debug, PartialEq)]
struct Rhs {
    z: i8,
    x: u32,
}

#[derive(Debug, PartialEq, StructuralConvert)]
#[convert(from = "Rhs")]
struct Lhs {
    z: i32,
    x: u32,
}

assert_eq!(Lhs { z: 1, x: 2 }, Rhs { z: 1, x: 2 }.into());
assert_eq!(Lhs { z: 1, x: 2 }, Rhs { z: 1, x: 2 }.into());
```

Generated code:

```rs
impl From<Rhs> for Lhs {
    fn from(value: Rhs) -> Self {
        match value {
            Rhs { z, x } => Lhs {
                z: z.into(),
                x: x.into(),
            },
        }
    }
}
```

### Enum

```rs
    #[derive(Debug, PartialEq)]
    enum Rhs {
        A { z: i8, x: u32 },
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(from = "Rhs")]
    enum Lhs {
        A { z: i32, x: u32 },
    }

    assert_eq!(Lhs::A { z: 1, x: 2 }, Rhs::A { z: 1, x: 2 }.into());
```

Generated code:

```rs
impl From<Rhs> for Lhs {
    fn from(value: Rhs) -> Self {
        match value {
            Rhs::A { z, x } => Lhs::A {
                z: z.into(),
                x: x.into(),
            },
        }
    }
}
```
