use darling::FromMeta;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::NestedMeta;
use syn::Path;

#[derive(Debug, Default, Clone)]
pub struct TryFromEnumVariantAttributes {
    pub target: Option<Path>,
    pub skip: bool,
    pub rename: Option<Ident>,
    pub default: bool,
}

impl FromMeta for TryFromEnumVariantAttributes {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut target: Option<Path> = None;
        let mut default: bool = Default::default();
        let mut rename: Option<Ident> = None;
        let mut skip: bool = Default::default();

        for (i, item) in items.iter().enumerate() {
            match item {
                NestedMeta::Meta(meta) => match meta {
                    syn::Meta::Path(meta_path) => {
                        match meta_path.to_token_stream().to_string().as_str() {
                            "default" => {
                                default = true;
                            }
                            "skip" => {
                                skip = true;
                            }
                            _ if i == 0 => target = Some(meta_path.clone()),
                            _ => {
                                return Err(darling::Error::custom(format!(
                                    "Path only allowed in the first argument: {}",
                                    meta.to_token_stream()
                                ))
                                .with_span(meta_path))
                            }
                        }
                    }
                    syn::Meta::List(list) => {
                        return Err(darling::Error::custom(format!(
                            "Attribute not supported: {}",
                            list.to_token_stream()
                        ))
                        .with_span(list))
                    }
                    syn::Meta::NameValue(name_value) => {
                        match name_value.path.to_token_stream().to_string().as_str() {
                            "rename" => {
                                rename = Some(Ident::from_value(&name_value.lit)?);
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

        Ok(Self {
            target,
            skip,
            rename,
            default,
        })
    }
}
