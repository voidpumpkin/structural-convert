use crate::structural_convert::on_fields_named::create_try_into_match_branch_for_fields_named::create_try_into_match_branch_for_fields_named;
use crate::structural_convert::on_fields_unnamed::on_fields_unnamed;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Fields;
use syn::Path;

pub(crate) fn create_try_into_impl_for_struct(
    from_path: &Path,
    struct_data: &DataStruct,
    into_path: &Path,
    skip_after: Option<usize>,
) -> TokenStream {
    let match_branches = match &struct_data.fields {
        Fields::Unit => {
            quote! {
                #from_path => #into_path
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            let field_tokens = on_fields_unnamed(fields_unnamed, skip_after);
            quote! {
                #from_path(#(#field_tokens,)* ..) => #into_path(#(#field_tokens.try_into().map_err(|_| "Failed to convert field".to_string())?,)*)
            }
        }
        Fields::Named(fields_named) => {
            create_try_into_match_branch_for_fields_named(from_path, fields_named, into_path)
        }
    };
    quote!(
        #[automatically_derived]
        impl TryFrom<#from_path> for #into_path {
            type Error = String;

            fn try_from(value: #from_path) -> Result<Self, Self::Error> {
                Ok(match value {
                    #match_branches
                })
            }
        }
    )
}
