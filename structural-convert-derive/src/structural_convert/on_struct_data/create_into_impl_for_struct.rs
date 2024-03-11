use crate::structural_convert::on_fields_named::create_into_match_branch_for_fields_named::create_into_match_branch_for_fields_named;
use crate::structural_convert::on_fields_unnamed::create_match_branch_for_fields_unnamed;
use crate::structural_convert::ConversionError;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Fields;
use syn::Path;

pub fn create_into_impl_for_struct(
    from_path: &Path,
    struct_data: &DataStruct,
    into_path: &Path,
    skip_after: Option<usize>,
    default_for_fields: &[Ident],
) -> darling::Result<TokenStream> {
    let match_branches = match &struct_data.fields {
        Fields::Unit => {
            quote! {
                #from_path => #into_path
            }
        }
        Fields::Unnamed(fields_unnamed) => create_match_branch_for_fields_unnamed(
            from_path,
            |field, _| quote! {#field.into()},
            into_path,
            fields_unnamed,
            skip_after,
            ConversionError::empty(),
        )?,
        Fields::Named(fields_named) => create_into_match_branch_for_fields_named(
            from_path,
            fields_named,
            into_path,
            default_for_fields,
        )?,
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
