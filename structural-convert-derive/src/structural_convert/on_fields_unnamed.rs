use std::ops::AddAssign;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::FieldsUnnamed;
use syn::Index;
use syn::Path;

use crate::structural_convert::on_field_type::recursive_type;
use crate::structural_convert::on_field_type::MyType;

use super::conversion_error::ConversionError;

/// item1, item2, item3 ...
pub fn create_match_branch_for_fields_unnamed(
    from_path: &Path,
    into_expr_fn: impl Fn(TokenStream, TokenStream) -> TokenStream,
    into_path: &Path,
    fields_unnamed: &FieldsUnnamed,
    skip_after: Option<usize>,
    conversion_error: ConversionError,
) -> darling::Result<TokenStream> {
    let take_len = skip_after.unwrap_or(fields_unnamed.unnamed.len());
    let (field_ident, into_expr): (Vec<_>, Vec<_>) = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let mut conversion_error = conversion_error.clone();
            let ident = format_ident!("item{i}");
            let mut into_expr = Default::default();

            conversion_error.from.unnamed(i);
            conversion_error.into.unnamed(i);

            into_expr = recursively_create_expr(
                into_expr,
                recursive_type(&field.ty)?,
                &quote!(#ident),
                &into_expr_fn,
                &mut conversion_error,
                0,
            );

            Ok((ident, into_expr))
        })
        .collect::<darling::Result<Vec<_>>>()?
        .into_iter()
        .take(take_len)
        .unzip();
    Ok(quote! {
        #from_path(#(#field_ident,)* ..) => #into_path(#(#into_expr,)*)
    })
}

pub fn recursively_create_expr(
    into_expr: TokenStream,
    my_type: MyType,
    ident: &TokenStream,
    into_expr_fn: &impl Fn(TokenStream, TokenStream) -> TokenStream,
    conversion_error: &mut ConversionError,
    mut i_count: usize,
) -> TokenStream {
    match my_type {
        MyType::Simple(_) => into_expr_fn(ident.clone(), conversion_error.to_token_stream()),
        MyType::Option(_, generic) => {
            conversion_error.from.enum_variant("Some");
            conversion_error.into.enum_variant("Some");
            conversion_error.from.unnamed(0);
            conversion_error.into.unnamed(0);

            let new_ident = format_ident!("some").to_token_stream();
            let into_expr = recursively_create_expr(
                into_expr,
                *generic,
                &new_ident,
                into_expr_fn,
                conversion_error,
                i_count,
            );
            quote!(match #ident {
                None => None,
                Some(#new_ident) => Some(#into_expr),
            })
        }
        MyType::List(_, generic) => {
            let i_ident = format_ident!("i{i_count}").to_token_stream();
            i_count.add_assign(1);
            conversion_error.from.dyn_unnamed(&i_ident);
            conversion_error.into.dyn_unnamed(&i_ident);

            let new_ident = format_ident!("li").to_token_stream();
            let into_expr = recursively_create_expr(
                into_expr,
                *generic,
                &new_ident,
                into_expr_fn,
                conversion_error,
                i_count,
            );
            quote!({
                let mut tmp = Vec::default();
                for (#i_ident, #new_ident) in #ident.into_iter().enumerate() {
                    tmp.push(#into_expr);
                }
                tmp.into_iter().collect()
            })
        }
        MyType::Result(_, g1, g2) => {
            let new_ok_ident = format_ident!("ok").to_token_stream();
            let into_ok_expr = {
                let mut conversion_error = conversion_error.clone();

                conversion_error.from.enum_variant("Ok");
                conversion_error.into.enum_variant("Ok");
                conversion_error.from.unnamed(0);
                conversion_error.into.unnamed(0);

                recursively_create_expr(
                    into_expr.clone(),
                    *g1,
                    &new_ok_ident,
                    into_expr_fn,
                    &mut conversion_error,
                    i_count,
                )
            };

            let new_err_ident = format_ident!("err").to_token_stream();
            let into_err_expr = {
                let mut conversion_error = conversion_error.clone();

                conversion_error.from.enum_variant("Err");
                conversion_error.into.enum_variant("Err");
                conversion_error.from.unnamed(0);
                conversion_error.into.unnamed(0);

                recursively_create_expr(
                    into_expr,
                    *g2,
                    &new_err_ident,
                    into_expr_fn,
                    &mut conversion_error,
                    i_count,
                )
            };

            quote!(match #ident {
                Ok(#new_ok_ident) => Ok(#into_ok_expr),
                Err(#new_err_ident) => Err(#into_err_expr),
            })
        }
        MyType::Map(_, g1, g2) => {
            let i_ident = format_ident!("i{i_count}").to_token_stream();
            i_count.add_assign(1);
            conversion_error.from.dyn_unnamed(&i_ident);
            conversion_error.into.dyn_unnamed(&i_ident);

            let new_key_ident = format_ident!("key").to_token_stream();
            let into_key_expr = {
                let mut conversion_error = conversion_error.clone();
                conversion_error.from.enum_variant("key");
                conversion_error.into.enum_variant("key");

                recursively_create_expr(
                    into_expr.clone(),
                    *g1,
                    &new_key_ident,
                    into_expr_fn,
                    &mut conversion_error,
                    i_count,
                )
            };

            let new_val_ident = format_ident!("val").to_token_stream();
            let into_val_expr = {
                let mut conversion_error = conversion_error.clone();
                conversion_error.from.named_str("value");
                conversion_error.into.named_str("value");

                recursively_create_expr(
                    into_expr,
                    *g2,
                    &new_val_ident,
                    into_expr_fn,
                    &mut conversion_error,
                    i_count,
                )
            };

            quote!({
                let mut tmp = Vec::default();
                for (#i_ident, (#new_key_ident, #new_val_ident)) in #ident.into_iter().enumerate() {
                    tmp.push((#into_key_expr, #into_val_expr));
                }
                tmp.into_iter().collect()
            })
        }
        MyType::Tuple(tt, members) => {
            let into_exprs = members.into_iter().enumerate().map(|(i, m)| {
                conversion_error.from.unnamed(i);
                conversion_error.into.unnamed(i);

                let i = Index {
                    index: i as u32,
                    span: tt.span(),
                };

                recursively_create_expr(
                    into_expr.clone(),
                    m,
                    &quote!(#ident.#i),
                    into_expr_fn,
                    conversion_error,
                    i_count,
                )
            });
            quote!((#(#into_exprs),*))
        }
    }
}
