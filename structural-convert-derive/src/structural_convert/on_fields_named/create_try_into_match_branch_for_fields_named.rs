use crate::structural_convert::ConversionError;
use crate::structural_convert::FieldNamedAttributes;
use darling::FromAttributes;
use darling::FromMeta;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use syn::FieldsNamed;
use syn::Path;
use syn::Type;

use super::create_match_branch_for_fields_named::create_match_branch_for_fields_named;
use super::create_match_branch_for_fields_named::FieldsNamedMatchBranchData;
use super::create_match_branch_for_fields_named::IntoFromPair;

#[derive(Debug, Default, Clone, FromMeta)]
#[darling(default)]
pub struct TryIntoFieldNamedAttributes {
    #[darling(rename = "for")]
    target: Option<Path>,
    rename: Option<Ident>,
    skip: bool,
    #[darling(rename = "as")]
    as_type: Option<Type>,
}

pub(crate) fn create_try_into_match_branch_for_fields_named(
    from_path: &Path,
    fields_named: &FieldsNamed,
    into_path: &Path,
    added_default_fields: &[Ident],
    conversion_error: ConversionError,
) -> darling::Result<TokenStream> {
    let match_branch_data = fields_named
        .named
        .iter()
        .filter_map(|f| {
            let Some(ident) = f.ident.as_ref() else {
                unreachable!()
            };
            let field_type = f.ty.clone();

            let attrs = match FieldNamedAttributes::from_attributes(&f.attrs) {
                Ok(ok) => ok,
                Err(err) => return Some(Err(err)),
            }
            .try_into;

            let default_attrs = attrs.iter().find(|e| e.target.is_none());
            let has_targeted_attrs = attrs.iter().any(|e| e.target.is_some());
            if default_attrs.is_some() && has_targeted_attrs {
                return Some(Err(darling::Error::custom(
                    "Mixing attributes with 'for' path and no path is not allowed",
                )
                .with_span(f)));
            }

            let skip = attrs.iter().any(|e| match &e.target {
                Some(target) if target == into_path => e.skip,
                Some(_) => false,
                None => e.skip,
            });
            if skip {
                return None;
            }

            let as_type = attrs.iter().find_map(|e| match &e.target {
                Some(target) if target == into_path => e.as_type.clone(),
                Some(_) => None,
                None => e.as_type.clone(),
            });

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

            Some(Ok(FieldsNamedMatchBranchData {
                lhs_field_name: Some(ident.clone()),
                into_from_pair,
                as_type,
                field_type,
            }))
        })
        .collect::<darling::Result<Vec<_>>>()?;

    create_match_branch_for_fields_named(
        from_path,
        |field_name, err| quote!(#field_name.try_into().map_err(|_| #err)?),
        into_path,
        match_branch_data,
        added_default_fields,
        conversion_error,
    )
}
