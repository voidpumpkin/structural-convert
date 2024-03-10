use crate::structural_convert::on_fields_named::create_try_from_match_branch_for_fields_named::create_try_from_match_branch_for_fields_named;
use crate::structural_convert::on_fields_unnamed::create_match_branch_for_fields_unnamed;
use crate::structural_convert::ConversionError;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Fields;
use syn::Path;

pub(crate) fn create_try_from_impl_for_struct(
    from_path: &Path,
    struct_data: &DataStruct,
    into_path: &Path,
) -> darling::Result<TokenStream> {
    let conversion_error = ConversionError::new(from_path, into_path);

    let match_branches = match &struct_data.fields {
        Fields::Unit => {
            quote! {
                #from_path => #into_path
            }
        }
        Fields::Unnamed(fields_unnamed) => create_match_branch_for_fields_unnamed(
            from_path,
            |field, err| quote! {#field.try_into().map_err(|_| #err)?},
            into_path,
            fields_unnamed,
            None,
            conversion_error,
        )?,
        Fields::Named(fields_named) => create_try_from_match_branch_for_fields_named(
            from_path,
            fields_named,
            into_path,
            conversion_error,
        )?,
    };
    Ok(quote!(
        #[automatically_derived]
        impl TryFrom<#from_path> for #into_path {
            type Error = String;

            fn try_from(value: #from_path) -> Result<Self, Self::Error> {
                Ok(match value {
                    #match_branches
                })
            }
        }
    ))
}
