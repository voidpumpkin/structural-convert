use syn::Ident;
use syn::Path;
use syn::PathSegment;

pub fn concat_enum_with_variant(enum_path: &Path, variant_ident: &Ident) -> Path {
    let segment = PathSegment {
        ident: variant_ident.clone(),
        arguments: Default::default(),
    };
    let from_path = Path {
        leading_colon: None,
        segments: enum_path
            .clone()
            .segments
            .into_iter()
            .chain(std::iter::once(segment))
            .collect(),
    };
    from_path
}
