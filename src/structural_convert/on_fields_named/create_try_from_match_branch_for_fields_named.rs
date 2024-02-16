
use crate::structural_convert::FieldNamedAttributes;
use darling::FromAttributes;
use darling::FromMeta;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;


use syn::FieldsNamed;
use syn::Path;

#[derive(Debug, Default, Clone, FromMeta)]
#[darling(default)]
pub struct TryFromFieldNamedAttributes {
    #[darling(rename = "for")]
    target: Option<Path>,
    rename: Option<Ident>,
}

pub(crate) fn create_try_from_match_branch_for_fields_named(
    from_path: &Path,
    fields_named: &FieldsNamed,

    into_path: &Path,
) -> TokenStream {
    let (from_field_ident, into_field_name): (Vec<_>, Vec<_>) = fields_named
        .named
        .iter()
        .map(|f| {
            let Some(ident) = f.ident.as_ref() else {
                unreachable!()
            };
            let attrs = FieldNamedAttributes::from_attributes(&f.attrs)
                .expect("Invalid field attributes")
                .try_from;

            let default_attrs = attrs.iter().find(|e| e.target.is_none());
            let has_targeted_attrs = attrs.iter().any(|e| e.target.is_some());
            if default_attrs.is_some() && has_targeted_attrs {
                panic!("For fields mixing attributes targeted and not targeted is not allowed");
            }

            let from_field_ident: Ident = attrs
                .iter()
                .find_map(|e| match &e.target {
                    Some(target) if target == from_path => e.rename.clone(),
                    Some(_) => None,
                    _ => e.rename.clone(),
                })
                .unwrap_or_else(|| ident.clone());

            (from_field_ident, ident)
        })
        .unzip();
    quote! {
        #from_path{
            #(#from_field_ident,)*
            ..
        } => #into_path{
            #(#into_field_name: #from_field_ident.try_into().map_err(|_| "Failed to convert field".to_string())?,)*
        }
    }
}
