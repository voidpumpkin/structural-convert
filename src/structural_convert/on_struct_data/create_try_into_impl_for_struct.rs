use crate::structural_convert::on_fields_named::create_try_into_match_branch_for_fields_named::create_try_into_match_branch_for_fields_named;
use crate::structural_convert::on_fields_unnamed::create_match_branch_for_fields_unnamed;

use proc_macro2::Ident;
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
    default_for_fields: &[Ident],
) -> TokenStream {
    let match_branches = match &struct_data.fields {
        Fields::Unit => {
            quote! {
                #from_path => #into_path
            }
        }
        Fields::Unnamed(fields_unnamed) => create_match_branch_for_fields_unnamed(
            &from_path,
            |field| quote! {#field.try_into().map_err(|_| "Failed to convert field".to_string())?},
            &into_path,
            fields_unnamed,
            skip_after,
        ),
        Fields::Named(fields_named) => create_try_into_match_branch_for_fields_named(
            from_path,
            fields_named,
            into_path,
            default_for_fields,
        ),
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
