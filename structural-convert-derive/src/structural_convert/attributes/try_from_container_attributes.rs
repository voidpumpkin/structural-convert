use darling::FromMeta;
use quote::ToTokens;
use syn::NestedMeta;
use syn::Path;

#[derive(Debug, Clone)]
pub struct TryFromContainerAttributes {
    pub path: Path,
}

impl FromMeta for TryFromContainerAttributes {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let Some(item) = items.get(0) else {
            return Err(darling::Error::custom("Missing arguments"));
        };
        match item {
            NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => Ok(Self { path: path.clone() }),
                syn::Meta::List(_) => Err(darling::Error::custom(format!(
                    "Meta List not supported: {}",
                    meta.to_token_stream()
                ))
                .with_span(item)),
                syn::Meta::NameValue(_) => Err(darling::Error::custom(format!(
                    "Meta NameValue not supported: {}",
                    meta.to_token_stream()
                ))
                .with_span(item)),
            },
            NestedMeta::Lit(_) => Err(darling::Error::custom("Expected Meta here").with_span(item)),
        }
    }
}
