use crate::structural_convert::on_fields_named::on_fields_named;
use crate::structural_convert::on_fields_unnamed::on_fields_unnamed;
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
    let match_branches = enum_data.variants.iter().map(|variant| {
        let variant_ident = variant.ident.clone();
        match &variant.fields {
            Fields::Unit => {
                quote! {
                    #from_path::#variant_ident => #into_path::#variant_ident.into()
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let field_tokens = on_fields_unnamed(fields_unnamed);
                quote! {
                    #from_path::#variant_ident(#(#field_tokens,)*) => #into_path::#variant_ident(#(#field_tokens.into(),)*)
                }
            }
            Fields::Named(fields_named) => {
                let field_tokens = on_fields_named(fields_named);
                quote! {
                    #from_path::#variant_ident{
                        #(#field_tokens,)*
                    } => #into_path::#variant_ident{
                        #(#field_tokens: #field_tokens.into(),)*
                    }
                }
            }
        }
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
