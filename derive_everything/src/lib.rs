// use impls::impls;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Nothing, parse_macro_input, Item};

#[proc_macro_attribute]
pub fn derive_everything(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as Nothing);
    let item = parse_macro_input!(input as Item);

    let attributes: Vec<_> = match &item {
        Item::Enum(item) => item
            .variants
            .iter()
            .flat_map(|variant| variant.fields.iter())
            .collect(),
        Item::Struct(item) => item.fields.iter().collect(),
        _ => panic!("Only structs and enums work with derive_everything."),
    };

    for &attribute in &attributes {
        let ty = &attribute.ty;
        // FIXME: This does not work.
        // let implements = impls!(ty: Debug);
        let _ = ty;
    }

    quote! {
        // TODO:
    }
    .into()
}
