use std::fmt;

use quote::ToTokens;
use syn::GenericArgument;
use syn::PathArguments;
use syn::Type;
use syn::TypePath;
use syn::TypeTuple;

#[derive(Clone)]
pub enum MyType {
    Simple(Type),
    Option(TypePath, Box<MyType>),
    Result(TypePath, Box<MyType>, Box<MyType>),
    List(TypePath, Box<MyType>),
    Map(TypePath, Box<MyType>, Box<MyType>),
    Tuple(TypeTuple, Vec<MyType>),
}

impl fmt::Debug for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyType::Simple(ty) => write!(f, "Simple({})", ty.to_token_stream()),
            MyType::Option(tp, g) => write!(f, "Option({}, {g:?})", tp.to_token_stream(),),
            MyType::List(tp, g) => {
                write!(f, "List({}, {g:?})", tp.to_token_stream())
            }
            MyType::Result(tp, g1, g2) => {
                write!(f, "Result({}, {g1:?}, {g2:?})", tp.to_token_stream(),)
            }
            MyType::Map(tp, g1, g2) => {
                write!(f, "Map({}, {g1:?}, {g2:?})", tp.to_token_stream(),)
            }
            MyType::Tuple(tt, members) => {
                write!(f, "Map({}, {members:?})", tt.to_token_stream(),)
            }
        }
    }
}

pub fn recursive_type(ty: &Type) -> darling::Result<MyType> {
    let simple = MyType::Simple(ty.clone());

    match &ty {
        Type::Path(typepath) if typepath.qself.is_none() => {
            let Some(end_path) = typepath.path.segments.last() else {
                return Err(darling::Error::unexpected_type(
                    &ty.to_token_stream().to_string(),
                ));
            };
            let end_name = end_path.ident.to_string();
            let generics = &end_path.arguments;

            (|| match end_name.as_str() {
                "Option" => {
                    let PathArguments::AngleBracketed(generics) = generics else {
                        return Ok(simple);
                    };
                    let Some(GenericArgument::Type(generic)) = generics.args.iter().next() else {
                        return Ok(simple);
                    };
                    let my_generic = recursive_type(generic)?;
                    Ok(MyType::Option(typepath.clone(), Box::new(my_generic)))
                }
                "Vec" | "VecDeque" | "LinkedList" | "BTreeSet" | "HashSet" | "BinaryHeap" => {
                    let PathArguments::AngleBracketed(generics) = generics else {
                        return Ok(simple);
                    };
                    let Some(GenericArgument::Type(generic)) = generics.args.iter().next() else {
                        return Ok(simple);
                    };
                    let my_generic = recursive_type(generic)?;
                    Ok(MyType::List(typepath.clone(), Box::new(my_generic)))
                }
                "Result" => {
                    let PathArguments::AngleBracketed(generics) = generics else {
                        return Ok(simple);
                    };
                    let mut generics = generics.args.iter();
                    let Some(GenericArgument::Type(generic1)) = generics.next() else {
                        return Ok(simple);
                    };
                    let Some(GenericArgument::Type(generic2)) = generics.next() else {
                        return Ok(simple);
                    };
                    let my_generic1 = recursive_type(generic1)?;
                    let my_generic2 = recursive_type(generic2)?;
                    Ok(MyType::Result(
                        typepath.clone(),
                        Box::new(my_generic1),
                        Box::new(my_generic2),
                    ))
                }
                "BTreeMap" | "HashMap" => {
                    let PathArguments::AngleBracketed(generics) = generics else {
                        return Ok(simple);
                    };
                    let mut generics = generics.args.iter();
                    let Some(GenericArgument::Type(generic1)) = generics.next() else {
                        return Ok(simple);
                    };
                    let Some(GenericArgument::Type(generic2)) = generics.next() else {
                        return Ok(simple);
                    };
                    let my_generic1 = recursive_type(generic1)?;
                    let my_generic2 = recursive_type(generic2)?;
                    Ok(MyType::Map(
                        typepath.clone(),
                        Box::new(my_generic1),
                        Box::new(my_generic2),
                    ))
                }
                _ => Ok(simple),
            })()
        }
        Type::Tuple(tuple) => {
            let membs = tuple
                .elems
                .iter()
                .map(recursive_type)
                .collect::<darling::Result<_>>()?;
            Ok(MyType::Tuple(tuple.clone(), membs))
        }
        _ => Ok(simple),
    }
}
