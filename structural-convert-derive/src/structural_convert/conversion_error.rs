use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use std::fmt;

#[derive(Clone)]
pub struct ConversionError {
    pub from: ConversionPath,
    pub into: ConversionPath,
}

impl ConversionError {
    pub fn new(from: impl ToTokens, into: impl ToTokens) -> ConversionError {
        Self {
            from: ConversionPath::start(from),
            into: ConversionPath::start(into),
        }
    }

    pub fn empty() -> ConversionError {
        Self {
            from: ConversionPath::empty(),
            into: ConversionPath::empty(),
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Failed to convert:")?;
        writeln!(f, "  from: {}", self.from)?;
        writeln!(f, "  into: {}", self.into)
    }
}

impl ToTokens for ConversionError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let err = self.to_string();
        tokens.append_all(quote!(format!(
            "{}\noriginal error:\n{}",
            format!(#err),
            err
        )))
    }
}

#[derive(Clone)]
pub struct ConversionPath {
    pub(crate) path: Vec<ConversionStep>,
}

impl fmt::Display for ConversionPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self
            .path
            .iter()
            .map(|p| p.to_string())
            .collect::<String>()
            .to_token_stream();
        write!(f, "{path}")
    }
}

impl ConversionPath {
    pub fn start(start_ident: impl ToTokens) -> ConversionPath {
        ConversionPath {
            path: vec![ConversionStep::StartIdent(
                start_ident.to_token_stream().to_string(),
            )],
        }
    }

    pub fn unnamed(&mut self, i: usize) {
        self.path.push(ConversionStep::UnnamedField(i));
    }

    pub fn named(&mut self, ident: impl ToTokens) {
        self.path.push(ConversionStep::NamedField(
            ident.to_token_stream().to_string(),
        ));
    }

    pub fn named_str(&mut self, ident: &str) {
        self.path
            .push(ConversionStep::NamedField(ident.to_string()));
    }

    pub fn enum_variant(&mut self, ident: &str) {
        self.path
            .push(ConversionStep::EnumVariant(ident.to_string()));
    }

    pub fn dyn_unnamed(&mut self, ident: impl ToTokens) {
        self.path.push(ConversionStep::DynamicUnnamed(
            ident.to_token_stream().to_string(),
        ));
    }

    pub fn empty() -> ConversionPath {
        ConversionPath { path: vec![] }
    }
}

#[derive(Clone)]
pub enum ConversionStep {
    StartIdent(String),
    UnnamedField(usize),
    NamedField(String),
    EnumVariant(String),
    DynamicUnnamed(String),
}

impl fmt::Display for ConversionStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionStep::StartIdent(ident) => write!(f, "{ident}"),
            ConversionStep::UnnamedField(ident) => write!(f, ".{ident}"),
            ConversionStep::NamedField(ident) => write!(f, ".{ident}"),
            ConversionStep::EnumVariant(ident) => write!(f, "::{ident}"),
            ConversionStep::DynamicUnnamed(ident) => write!(f, ".{{{ident}}}"),
        }
    }
}
