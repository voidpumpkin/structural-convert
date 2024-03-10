use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;
use syn::Type;

use crate::structural_convert::on_field_type::recursive_type;
use crate::structural_convert::on_field_type::MyType;
use crate::structural_convert::on_fields_unnamed::recursively_create_expr;
use crate::structural_convert::ConversionError;

/// Expected to become these tokens:
/// #from_path{
///     #(#lhs_field_name,)*
///     ..
/// } => #into_path{
///     #(#into_field_name: #from_field_expr,)* // This is into_from_pair
/// }
/// aka
/// ```compile_fail
/// SomeStructA { x, a, ..} => SomeStructB {
///     x: x.into(),
///     z: a.into(),
///     y: Default::default(),
/// }
/// ```
pub struct FieldsNamedMatchBranchData {
    pub lhs_field_name: Option<Ident>,
    pub into_from_pair: IntoFromPair,
    pub as_type: Option<Type>,
    pub field_type: Type,
}

/// Expected to become these tokens:
/// #(#into_field_name: #from_field_ident.into(),)*
/// aka
/// ```compile_fail
/// x: y.into(),
/// z: z.into(),
/// y: Default::default(),
/// ```
pub struct IntoFromPair {
    pub into_field_name: Ident,
    pub from_field_ident: Option<Ident>,
}

pub fn create_match_branch_for_fields_named(
    from_path: &Path,
    into_expr: impl Fn(TokenStream, TokenStream) -> TokenStream,
    into_path: &Path,
    mut match_branch_data: Vec<FieldsNamedMatchBranchData>,
    added_default_fields: &[Ident],
    conversion_error: ConversionError,
) -> darling::Result<TokenStream> {
    for default_field_name in added_default_fields {
        match_branch_data.push(FieldsNamedMatchBranchData {
            lhs_field_name: None,
            into_from_pair: IntoFromPair {
                into_field_name: default_field_name.clone(),
                from_field_ident: None,
            },
            as_type: None,
            field_type: Type::Verbatim(Default::default()),
        })
    }

    let mut lhs_field_name = vec![];
    let mut into_field_name = vec![];
    let mut from_field_expr = vec![];
    for item in match_branch_data.into_iter() {
        let mut conversion_error = conversion_error.clone();

        if let Some(field_name) = item.lhs_field_name {
            lhs_field_name.push(field_name);

            conversion_error
                .from
                .named(&item.into_from_pair.into_field_name);
        }

        conversion_error
            .into
            .named(&item.into_from_pair.into_field_name);
        into_field_name.push(item.into_from_pair.into_field_name);

        let mut expr = quote!(Default::default());
        let mut tiep = item.field_type.clone();

        if let Some(field_name) = item.into_from_pair.from_field_ident {
            expr = Default::default();
            let mut identy = quote!(#field_name);

            if let Some(as_type) = item.as_type {
                let from_type = recursive_type(&tiep)?;

                tiep = as_type.clone();

                let mut parsed_as_type = recursive_type(&as_type)?;

                // From<Q> for Option<Q> is implemented
                if let (MyType::Simple(_), MyType::Option(_, inner)) = (&from_type, &parsed_as_type)
                {
                    parsed_as_type = *inner.clone();
                }

                let as_expr = recursively_create_expr(
                    expr.clone(),
                    parsed_as_type,
                    &identy,
                    &into_expr,
                    &mut conversion_error,
                    0,
                );

                identy = quote!({
                    let temp: #as_type = #as_expr;
                    temp
                });
            }

            expr = recursively_create_expr(
                expr,
                recursive_type(&tiep)?,
                &identy,
                &into_expr,
                &mut conversion_error,
                0,
            );
        }

        from_field_expr.push(expr);
    }

    Ok(quote! {
        #from_path{
            #(#lhs_field_name,)*
            ..
        } => #into_path{
            #(#into_field_name: #from_field_expr,)*
        }
    })
}
