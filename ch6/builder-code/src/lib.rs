use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();

    let name = ast.ident;
    let builder = format_ident!("{}Builder", name);

    quote! {
        struct #builder {

        }
    }
}
