//! Derive everything built-in.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Nothing, parse_macro_input, Item};

/// Derive everything built-in for a `struct`, except `Copy`.
///
/// Equivalent to `#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]`
#[proc_macro_attribute]
pub fn derive_everything(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as Nothing);
    let item = parse_macro_input!(input as Item);

    quote! {
        #[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #item
    }
    .into()
}

/// Useful when deriving everything for an `enum`.
///
/// Equivalent to `#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]`
#[proc_macro_attribute]
pub fn derive_enum_everything(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as Nothing);
    let item = parse_macro_input!(input as Item);

    quote! {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #item
    }
    .into()
}

/// Useful when deriving everything for a `struct` containing floats.
///
/// Equivalent to `#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]`
#[proc_macro_attribute]
pub fn derive_float_everything(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as Nothing);
    let item = parse_macro_input!(input as Item);

    quote! {
        #[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
        #item
    }
    .into()
}

/// Useful when deriving everything for an `enum` containing floats.
///
/// Equivalent to `#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]`
#[proc_macro_attribute]
pub fn derive_float_enum_everything(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as Nothing);
    let item = parse_macro_input!(input as Item);

    quote! {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        #item
    }
    .into()
}
