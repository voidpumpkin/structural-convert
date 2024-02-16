
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
pub struct IntoFieldNamedAttributes {
    #[darling(rename = "for")]
    target: Option<Path>,
    rename: Option<Ident>,
}

pub(crate) fn create_into_match_branch_for_fields_named(
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
                .into;

            let default_attrs = attrs.iter().find(|e| e.target.is_none());
            let has_targeted_attrs = attrs.iter().any(|e| e.target.is_some());
            if default_attrs.is_some() && has_targeted_attrs {
                panic!("For fields mixing attributes targeted and not targeted is not allowed");
            }

            let into_field_ident: Ident = attrs
                .iter()
                .find_map(|e| match &e.target {
                    Some(target) if target == into_path => e.rename.clone(),
                    Some(_) => None,
                    _ => e.rename.clone(),
                })
                .unwrap_or_else(|| ident.clone());

            (ident, into_field_ident)
        })
        .unzip();
    quote! {
        #from_path{
            #(#from_field_ident,)*
            ..
        } => #into_path{
            #(#into_field_name: #from_field_ident.into(),)*
        }
    }
}