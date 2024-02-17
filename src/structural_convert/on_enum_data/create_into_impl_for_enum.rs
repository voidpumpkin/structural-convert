use crate::structural_convert::on_enum_data::utils::concat_enum_with_variant;
use crate::structural_convert::on_fields_named::create_into_match_branch_for_fields_named::create_into_match_branch_for_fields_named;

use crate::structural_convert::on_fields_unnamed::on_fields_unnamed;
use crate::structural_convert::EnumVariantAttributes;
use darling::FromAttributes;
use darling::FromMeta;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;
use syn::Fields;
use syn::Path;

#[derive(Debug, Default, Clone, FromMeta)]
#[darling(default)]
pub struct IntoEnumVariantAttributes {
    #[darling(rename = "for")]
    target: Option<Path>,
    rename: Option<Ident>,
    skip_after: Option<usize>,
    skip: bool,
}

pub(crate) fn create_into_impl_for_enum(
    from_path: &Path,
    enum_data: &DataEnum,
    into_path: &Path,
    default_for_fields: &[Ident],
    default: bool,
) -> TokenStream {
    let mut catch_all_branches = vec![];

    if default {
        catch_all_branches.push(quote! { _ => #into_path::default()})
    }

    let match_branches = enum_data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = variant.ident.clone();
            let from_variant_ident = &variant_ident;
            let attrs = EnumVariantAttributes::from_attributes(&variant.attrs)
                .expect("Invalid field attributes")
                .into;

            let default_attrs = attrs.iter().find(|e| e.target.is_none());
            let has_targeted_attrs = attrs.iter().any(|e| e.target.is_some());
            if default_attrs.is_some() && has_targeted_attrs {
                panic!("For fields mixing attributes targeted and not targeted is not allowed");
            }
            let skip = attrs.iter().any(|e| match &e.target {
                Some(target) if target == into_path => e.skip,
                Some(_) => false,
                None => e.skip,
            });
            if skip {
                return None;
            }

            let skip_after = attrs.iter().find_map(|e| match &e.target {
                Some(target) if target == into_path => e.skip_after,
                Some(_) => None,
                None => e.skip_after,
            });

            let into_variant_ident: &Ident = attrs
                .iter()
                .find_map(|e| match &e.target {
                    Some(target) if target == into_path => e.rename.as_ref(),
                    Some(_) => None,
                    _ => e.rename.as_ref(),
                })
                .unwrap_or(&variant_ident);

            let from_path = concat_enum_with_variant(from_path, from_variant_ident);
            let into_path = concat_enum_with_variant(into_path, into_variant_ident);

            let branch = match &variant.fields {
                Fields::Unit => {
                    quote! {
                        #from_path => #into_path.into()
                    }
                }
                Fields::Unnamed(fields_unnamed) => {
                    let field_tokens = on_fields_unnamed(fields_unnamed, skip_after);
                    quote! {
                        #from_path(#(#field_tokens,)* ..) => #into_path(#(#field_tokens.into(),)*)
                    }
                }
                Fields::Named(fields_named) => create_into_match_branch_for_fields_named(
                    &from_path,
                    fields_named,
                    &into_path,
                    default_for_fields,
                ),
            };
            Some(branch)
        })
        .collect::<Vec<_>>();
    quote!(
        #[automatically_derived]
        impl From<#from_path> for #into_path {
            fn from(value: #from_path) -> Self {
                match value {
                    #(#match_branches),*
                    #(#catch_all_branches),*
                }
            }
        }
    )
}
