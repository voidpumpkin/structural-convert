use crate::structural_convert::on_fields_named::on_fields_named;
use crate::structural_convert::on_fields_unnamed::on_fields_unnamed;
use crate::structural_convert::TryFromFieldAttributes;
use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;
use syn::Fields;
use syn::Path;

pub(crate) fn create_try_from_impl_for_enum(
    from_path: &Path,
    enum_data: &DataEnum,
    into_path: &Path,
) -> TokenStream {
    let match_branches = enum_data.variants.iter().filter_map(|variant| {
        let variant_ident = variant.ident.clone();
        let attrs = TryFromFieldAttributes::from_attributes(&variant.attrs).expect("Invalid field attributes").try_from;
        let default_attrs = attrs.iter().find(|e| e.target.is_none());
        let has_targeted_attrs = attrs.iter().any(|e|e.target.is_some());
        if default_attrs.is_some() && has_targeted_attrs {
            panic!("For fields mixing attributes targeted and not targeted is not allowed");
        }
        let skip = attrs.iter().any(|e| match &e.target {
            Some(target) if target == from_path => e.skip,
            Some(_) => false,
            None => e.skip,
        });
        if skip {
            return None;
        }
        let branch = match &variant.fields {
            Fields::Unit => {
                quote! {
                    #from_path::#variant_ident => #into_path::#variant_ident.try_into().map_err(|_| "Failed to convert field".to_string())?
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let field_tokens = on_fields_unnamed(fields_unnamed);
                quote! {
                    #from_path::#variant_ident(#(#field_tokens,)* ..) => #into_path::#variant_ident(#(#field_tokens.try_into().map_err(|_| "Failed to convert field".to_string())?,)*)
                }
            }
            Fields::Named(fields_named) => {
                let field_tokens = on_fields_named(fields_named);
                quote! {
                    #from_path::#variant_ident{
                        #(#field_tokens,)*
                        ..
                    } => #into_path::#variant_ident{
                        #(#field_tokens: #field_tokens.try_into().map_err(|_| "Failed to convert field".to_string())?,)*
                    }
                }
            }
        };
        Some(branch)
    }).collect::<Vec<_>>();
    quote!(
        #[automatically_derived]
        impl TryFrom<#from_path> for #into_path {
            type Error = String;

            fn try_from(value: #from_path) -> Result<Self, Self::Error> {
                Ok(match value {
                    #(#match_branches,)*
                })
            }
        }
    )
}
