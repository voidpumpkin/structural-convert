use darling::FromMeta;
use proc_macro2::Ident;
use quote::format_ident;
use quote::ToTokens;
use syn::Lit;
use syn::NestedMeta;

#[derive(Debug, Clone, Default)]
pub struct IdentAsLiteralList(pub Vec<Ident>);

impl FromMeta for IdentAsLiteralList {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut idents: Vec<Ident> = vec![];
        for item in items {
            match item {
                NestedMeta::Meta(_) => {
                    return Err(
                        darling::Error::custom("Expected list of literals here").with_span(item)
                    );
                }
                NestedMeta::Lit(lit) => {
                    let ident = match lit {
                        Lit::Str(inner_lit) => format_ident!("{}", inner_lit.value()),
                        Lit::ByteStr(inner_lit) => {
                            let s = String::from_utf8(inner_lit.value()).map_err(|_| {
                                darling::Error::unsupported_shape(
                                    lit.to_token_stream().to_string().as_str(),
                                )
                            })?;
                            format_ident!("{s}",)
                        }
                        Lit::Byte(inner_lit) => {
                            format_ident!("{}", (inner_lit.value() as char).to_string())
                        }
                        Lit::Char(inner_lit) => {
                            format_ident!("{}", inner_lit.value().to_string())
                        }
                        _ => {
                            return Err(darling::Error::custom(format!(
                                "Literal not supported: {}",
                                lit.to_token_stream()
                            ))
                            .with_span(item))
                        }
                    };
                    idents.push(ident);
                }
            }
        }

        Ok(Self(idents))
    }
}
