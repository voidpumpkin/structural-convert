use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Path;

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
    pub is_option: bool,
    pub as_type: Option<Path>,
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
    wrapper: impl Fn(TokenStream, Path) -> TokenStream,
    into_expr: impl Fn(TokenStream) -> TokenStream,
    into_path: &Path,
    mut match_branch_data: Vec<FieldsNamedMatchBranchData>,
    added_default_fields: &[Ident],
) -> TokenStream {
    for default_field_name in added_default_fields {
        match_branch_data.push(FieldsNamedMatchBranchData {
            lhs_field_name: None,
            into_from_pair: IntoFromPair {
                into_field_name: default_field_name.clone(),
                from_field_ident: None,
            },
            is_option: false,
            as_type: None,
        })
    }

    let mut lhs_field_name = vec![];
    let mut into_field_name = vec![];
    let mut from_field_expr = vec![];
    for item in match_branch_data.into_iter() {
        if let Some(field_name) = item.lhs_field_name {
            lhs_field_name.push(field_name);
        }

        into_field_name.push(item.into_from_pair.into_field_name);

        let mut expr = quote!(Default::default());

        if let Some(field_name) = item.into_from_pair.from_field_ident {
            expr = into_expr(field_name.to_token_stream());

            expr = match (item.is_option, item.as_type) {
                (false, None) => expr,
                (false, Some(as_type)) => into_expr(wrapper(field_name.to_token_stream(), as_type)),
                (true, None) => {
                    quote!(
                        match #field_name {
                            None => None,
                            Some(#field_name) => Some(#expr),
                        }
                    )
                }
                (true, Some(as_type)) => {
                    let match_expr = wrapper(field_name.to_token_stream(), as_type);
                    quote!(
                        match #match_expr {
                            None => None,
                            Some(#field_name) => Some(#expr),
                        }
                    )
                }
            };
        }

        from_field_expr.push(expr);
    }

    quote! {
        #from_path{
            #(#lhs_field_name,)*
            ..
        } => #into_path{
            #(#into_field_name: #from_field_expr,)*
        }
    }
}
