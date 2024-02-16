use crate::structural_convert::on_fields_named::on_fields_named;
use crate::structural_convert::on_fields_unnamed::on_fields_unnamed;
use crate::structural_convert::FromFieldAttributes;
use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;
use syn::Fields;
use syn::Path;

pub(crate) fn create_from_impl_for_enum(
    from_path: &Path,
    enum_data: &DataEnum,
    into_path: &Path,
) -> TokenStream {
    let match_branches = enum_data.variants.iter().filter_map(|variant| {
        let variant_ident = variant.ident.clone();
        let attrs = FromFieldAttributes::from_attributes(&variant.attrs).expect("Invalid field attributes").from;
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
                    #from_path::#variant_ident => #into_path::#variant_ident.into()
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let field_tokens = on_fields_unnamed(fields_unnamed);
                quote! {
                    #from_path::#variant_ident(#(#field_tokens,)* ..) => #into_path::#variant_ident(#(#field_tokens.into(),)*)
                }
            }
            Fields::Named(fields_named) => {
                let field_tokens = on_fields_named(fields_named);
                quote! {
                    #from_path::#variant_ident{
                        #(#field_tokens,)*
                        ..
                    } => #into_path::#variant_ident{
                        #(#field_tokens: #field_tokens.into(),)*
                    }
                }
            }
        };
        Some(branch)
    }).collect::<Vec<_>>();
    quote!(
        #[automatically_derived]
        impl From<#from_path> for #into_path {
            fn from(value: #from_path) -> Self {
                match value {
                    #(#match_branches,)*
                }
            }
        }
    )
}
