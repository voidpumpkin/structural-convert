use crate::structural_convert::create_from_impl_for_struct::create_from_impl_for_struct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Path;

pub(crate) fn on_struct_data(
    input_ident_path: &Path,
    struct_data: &DataStruct,
    into_paths: &[Path],
    from_paths: &[Path],
) -> TokenStream {
    let into_tokens = into_paths
        .iter()
        .map(|into_path| create_from_impl_for_struct(input_ident_path, struct_data, into_path))
        .collect::<Vec<_>>();
    let from_tokens = from_paths
        .iter()
        .map(|from_path| create_from_impl_for_struct(from_path, struct_data, input_ident_path))
        .collect::<Vec<_>>();

    quote!(
        #(
            #into_tokens
        )*
        #(
            #from_tokens
        )*
    )
}
