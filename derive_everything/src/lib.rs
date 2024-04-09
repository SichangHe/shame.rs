use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Nothing, parse_macro_input, Item};

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
