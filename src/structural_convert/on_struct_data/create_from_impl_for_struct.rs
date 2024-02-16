use crate::structural_convert::on_fields_named::on_fields_named;
use crate::structural_convert::on_fields_unnamed::on_fields_unnamed;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Fields;
use syn::Path;

pub(crate) fn create_from_impl_for_struct(
    from_path: &Path,
    struct_data: &DataStruct,
    into_path: &Path,
) -> TokenStream {
    let match_branches = match &struct_data.fields {
        Fields::Unit => {
            quote! {
                #from_path => #into_path
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            let field_tokens = on_fields_unnamed(fields_unnamed);
            quote! {
                #from_path(#(#field_tokens,)* ..) => #into_path(#(#field_tokens.into(),)*)
            }
        }
        Fields::Named(fields_named) => {
            let field_tokens = on_fields_named(fields_named);
            quote! {
                #from_path{
                    #(#field_tokens,)*
                    ..
                } => #into_path{
                    #(#field_tokens: #field_tokens.into(),)*
                }
            }
        }
    };
    quote!(
        #[automatically_derived]
        impl From<#from_path> for #into_path {
            fn from(value: #from_path) -> Self {
                match value {
                    #match_branches
                }
            }
        }
    )
}
