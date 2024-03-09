use crate::structural_convert::on_fields_named::create_from_match_branch_for_fields_named::create_from_match_branch_for_fields_named;
use crate::structural_convert::on_fields_unnamed::create_match_branch_for_fields_unnamed;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Fields;
use syn::Path;

pub(crate) fn create_from_impl_for_struct(
    from_path: &Path,
    struct_data: &DataStruct,
    into_path: &Path,
) -> darling::Result<TokenStream> {
    let match_branches = match &struct_data.fields {
        Fields::Unit => {
            quote! {
                #from_path => #into_path
            }
        }
        Fields::Unnamed(fields_unnamed) => create_match_branch_for_fields_unnamed(
            from_path,
            |field| quote! {#field.into()},
            into_path,
            fields_unnamed,
            None,
        )?,
        Fields::Named(fields_named) => {
            create_from_match_branch_for_fields_named(from_path, fields_named, into_path)?
        }
    };
    Ok(quote!(
        #[automatically_derived]
        impl From<#from_path> for #into_path {
            fn from(value: #from_path) -> Self {
                match value {
                    #match_branches
                }
            }
        }
    ))
}
