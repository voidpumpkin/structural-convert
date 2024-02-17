use crate::structural_convert::FieldNamedAttributes;
use darling::FromAttributes;
use darling::FromMeta;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use syn::FieldsNamed;
use syn::Path;

use super::create_match_branch_for_fields_named::create_match_branch_for_fields_named;
use super::create_match_branch_for_fields_named::FieldsNamedMatchBranchData;
use super::create_match_branch_for_fields_named::IntoFromPair;

#[derive(Debug, Default, Clone, FromMeta)]
#[darling(default)]
pub struct IntoFieldNamedAttributes {
    #[darling(rename = "for")]
    target: Option<Path>,
    rename: Option<Ident>,
    skip: bool,
    default: bool,
}

pub(crate) fn create_into_match_branch_for_fields_named(
    from_path: &Path,
    fields_named: &FieldsNamed,
    into_path: &Path,
    added_default_fields: &[Ident],
) -> TokenStream {
    let match_branch_data = fields_named
        .named
        .iter()
        .filter_map(|f| {
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

            let skip = attrs.iter().any(|e| match &e.target {
                Some(target) if target == into_path => e.skip,
                Some(_) => false,
                None => e.skip,
            });
            if skip {
                return None;
            }

            let into_field_ident: Ident = attrs
                .iter()
                .find_map(|e| match &e.target {
                    Some(target) if target == into_path => e.rename.clone(),
                    Some(_) => None,
                    _ => e.rename.clone(),
                })
                .unwrap_or_else(|| ident.clone());

            let into_from_pair = IntoFromPair {
                into_field_name: into_field_ident.clone(),
                from_field_ident: Some(ident.clone()),
            };

            Some(FieldsNamedMatchBranchData {
                lhs_field_name: Some(ident.clone()),
                into_from_pair,
            })
        })
        .collect::<Vec<_>>();

    create_match_branch_for_fields_named(
        from_path,
        |field_name| quote!(#field_name.into()),
        into_path,
        match_branch_data,
        added_default_fields,
    )
}
