use darling::FromMeta;
use quote::ToTokens;
use syn::NestedMeta;
use syn::Path;

use super::ident_as_literal_list::IdentAsLiteralList;

#[derive(Debug, Clone)]
pub struct IntoContainerAttributes {
    pub path: Path,
    pub skip_after: Option<usize>,
    pub default_for_fields: IdentAsLiteralList,
    pub default: bool,
}

impl FromMeta for IntoContainerAttributes {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut path: Option<Path> = None;
        let mut skip_after: Option<usize> = Default::default();
        let mut default_for_fields: IdentAsLiteralList = Default::default();
        let mut default: bool = Default::default();

        for (i, item) in items.iter().enumerate() {
            match item {
                NestedMeta::Meta(meta) => match meta {
                    syn::Meta::Path(meta_path) if i == 0 => path = Some(meta_path.clone()),
                    syn::Meta::Path(meta_path)
                        if meta_path.to_token_stream().to_string().as_str() == "default" =>
                    {
                        default = true;
                    }
                    syn::Meta::Path(_) => {
                        return Err(darling::Error::custom(format!(
                            "Path only allowed in the first argument: {}",
                            meta.to_token_stream()
                        ))
                        .with_span(item))
                    }
                    syn::Meta::List(list) => {
                        let items = list
                            .nested
                            .iter()
                            .map(ToOwned::to_owned)
                            .collect::<Vec<_>>();

                        match list.path.to_token_stream().to_string().as_str() {
                            "default_for_fields" => {
                                default_for_fields =
                                    IdentAsLiteralList::from_list(items.as_slice())?;
                            }
                            _ => {
                                return Err(darling::Error::custom(format!(
                                    "Attribute not supported: {}",
                                    list.to_token_stream()
                                ))
                                .with_span(list))
                            }
                        }
                    }
                    syn::Meta::NameValue(name_value) => {
                        match name_value.path.to_token_stream().to_string().as_str() {
                            "skip_after" => {
                                skip_after = Some(u32::from_value(&name_value.lit)? as usize);
                            }
                            _ => {
                                return Err(darling::Error::custom(format!(
                                    "Attribute not supported: {}",
                                    name_value.to_token_stream()
                                ))
                                .with_span(name_value))
                            }
                        }
                    }
                },
                NestedMeta::Lit(_) => {
                    return Err(darling::Error::custom("Expected Meta here").with_span(item));
                }
            }
        }

        let Some(path) = path else {
            return Err(darling::Error::missing_field("No path provided"));
        };

        Ok(Self {
            path,
            skip_after,
            default_for_fields,
            default,
        })
    }
}
