use std::collections::HashMap;

use structural_convert::StructuralConvert;

#[test]
fn unnamed_fields_option() {
    #[derive(Debug, PartialEq)]
    struct Rhs(Option<i8>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs(Option<i32>);

    assert_eq!(Lhs(Some(1)), Rhs(Some(1)).try_into().unwrap());
}

#[test]
fn unnamed_fields_vec() {
    #[derive(Debug, PartialEq)]
    struct Rhs(Vec<i8>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs(Vec<i32>);

    assert_eq!(Lhs(vec![1]), Rhs(vec![1]).try_into().unwrap());
}

#[test]
fn unnamed_fields_tuple() {
    #[derive(Debug, PartialEq)]
    struct Rhs((i8, i8));

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs((i32, i8));

    assert_eq!(Lhs((1, 2)), Rhs((1, 2)).try_into().unwrap());
}

#[test]
fn unnamed_fields_vec_tuples() {
    #[derive(Debug, PartialEq)]
    struct Rhs(Vec<(i8, i8)>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs(Vec<(i32, i8)>);

    assert_eq!(Lhs(vec![(1, 2)]), Rhs(vec![(1, 2)]).try_into().unwrap());
}

#[test]
fn unnamed_fields_option_vec() {
    #[derive(Debug, PartialEq)]
    struct Rhs(Option<Vec<i8>>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs(Option<Vec<i32>>);

    assert_eq!(Lhs(Some(vec![1])), Rhs(Some(vec![1])).try_into().unwrap());
}

#[test]
fn unnamed_fields_vec_option() {
    #[derive(Debug, PartialEq)]
    struct Rhs(Vec<Option<i8>>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs(Vec<Option<i32>>);

    assert_eq!(Lhs(vec![Some(1)]), Rhs(vec![Some(1)]).try_into().unwrap());
}

#[test]
fn unnamed_fields_vec_option_tuple() {
    #[derive(Debug, PartialEq)]
    struct Rhs(Vec<Option<(i32, i8)>>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs(Vec<Option<(i32, i8)>>);

    assert_eq!(
        Lhs(vec![Some((1, 2))]),
        Rhs(vec![Some((1, 2))]).try_into().unwrap()
    );
}

#[test]
fn unnamed_fields_vec_tuple_option() {
    #[derive(Debug, PartialEq)]
    struct Rhs(Vec<(Option<i8>, Option<i8>)>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs(Vec<(Option<i8>, Option<i8>)>);

    assert_eq!(
        Lhs(vec![(Some(1), Some(1))]),
        Rhs(vec![(Some(1), Some(1))]).try_into().unwrap()
    );
}

#[test]
fn unnamed_fields_tuple_vec_option() {
    #[derive(Debug, PartialEq)]
    struct Rhs((Vec<Option<i8>>, Vec<Option<i8>>));

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs((Vec<Option<i8>>, Vec<Option<i8>>));

    assert_eq!(
        Lhs((vec![Some(1)], vec![Some(1)])),
        Rhs((vec![Some(1)], vec![Some(1)])).try_into().unwrap()
    );
}

#[test]
fn unnamed_fields_result() {
    #[derive(Debug, PartialEq)]
    struct Rhs(Result<i8, u8>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs(Result<i32, u32>);

    assert_eq!(Lhs(Ok(1)), Rhs(Ok(1)).try_into().unwrap());
    assert_eq!(Lhs(Err(1)), Rhs(Err(1)).try_into().unwrap());
}

#[test]
fn unnamed_fields_hash_map() {
    #[derive(Debug, PartialEq)]
    struct Rhs(HashMap<u8, i8>);

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs(HashMap<u32, i32>);

    let mut lhs = HashMap::new();
    lhs.insert(1, 2);

    let mut rhs = HashMap::new();
    rhs.insert(1, 2);

    assert_eq!(Lhs(lhs), Rhs(rhs).try_into().unwrap());
}

#[test]
fn named_fields_option() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: Option<i8>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: Option<i32>,
    }

    assert_eq!(
        Lhs { item: Some(1) },
        Rhs { item: Some(1) }.try_into().unwrap()
    );
}

#[test]
fn named_fields_vec() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: Vec<i8>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: Vec<i32>,
    }

    assert_eq!(
        Lhs { item: vec![1] },
        Rhs { item: vec![1] }.try_into().unwrap()
    );
}

#[test]
fn named_fields_tuple() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: (i8, i8),
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: (i32, i8),
    }

    assert_eq!(
        Lhs { item: (1, 2) },
        Rhs { item: (1, 2) }.try_into().unwrap()
    );
}

#[test]
fn named_fields_vec_tuples() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: Vec<(i8, i8)>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: Vec<(i32, i8)>,
    }

    assert_eq!(
        Lhs { item: vec![(1, 2)] },
        Rhs { item: vec![(1, 2)] }.try_into().unwrap()
    );
}

#[test]
fn named_fields_option_vec() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: Option<Vec<i8>>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: Option<Vec<i32>>,
    }

    assert_eq!(
        Lhs {
            item: Some(vec![1])
        },
        Rhs {
            item: Some(vec![1])
        }
        .try_into()
        .unwrap()
    );
}

#[test]
fn named_fields_vec_option() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: Vec<Option<i8>>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: Vec<Option<i32>>,
    }

    assert_eq!(
        Lhs {
            item: vec![Some(1)]
        },
        Rhs {
            item: vec![Some(1)]
        }
        .try_into()
        .unwrap()
    );
}

#[test]
fn named_fields_vec_option_tuple() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: Vec<Option<(i32, i8)>>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: Vec<Option<(i32, i8)>>,
    }

    assert_eq!(
        Lhs {
            item: vec![Some((1, 2))]
        },
        Rhs {
            item: vec![Some((1, 2))]
        }
        .try_into()
        .unwrap()
    )
}

#[test]
fn named_fields_vec_tuple_option() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: Vec<(Option<i8>, Option<i8>)>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: Vec<(Option<i8>, Option<i8>)>,
    }

    assert_eq!(
        Lhs {
            item: vec![(Some(1), Some(1))]
        },
        Rhs {
            item: vec![(Some(1), Some(1))]
        }
        .try_into()
        .unwrap()
    )
}

#[test]
fn named_fields_tuple_vec_option() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: (Vec<Option<i8>>, Vec<Option<i8>>),
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: (Vec<Option<i8>>, Vec<Option<i8>>),
    }

    assert_eq!(
        Lhs {
            item: (vec![Some(1)], vec![Some(1)])
        },
        Rhs {
            item: (vec![Some(1)], vec![Some(1)])
        }
        .try_into()
        .unwrap()
    )
}

#[test]
fn named_fields_result() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: Result<i8, u8>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: Result<i32, u32>,
    }

    assert_eq!(Lhs { item: Ok(1) }, Rhs { item: Ok(1) }.try_into().unwrap());
    assert_eq!(
        Lhs { item: Err(1) },
        Rhs { item: Err(1) }.try_into().unwrap()
    );
}

#[test]
fn named_fields_hash_map() {
    #[derive(Debug, PartialEq)]
    struct Rhs {
        item: HashMap<u8, i8>,
    }

    #[derive(Debug, PartialEq, StructuralConvert)]
    #[convert(try_from(path = "Rhs"))]
    struct Lhs {
        item: HashMap<u32, i32>,
    }

    let mut lhs = HashMap::new();
    lhs.insert(1, 2);

    let mut rhs = HashMap::new();
    rhs.insert(1, 2);

    assert_eq!(Lhs { item: lhs }, Rhs { item: rhs }.try_into().unwrap());
}
