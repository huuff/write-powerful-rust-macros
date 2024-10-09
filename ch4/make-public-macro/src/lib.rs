use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ast = parse_macro_input!(item as DeriveInput);

    let public_version = quote! {};

    public_version.into()
}
