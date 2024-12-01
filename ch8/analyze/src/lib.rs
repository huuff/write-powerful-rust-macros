#![allow(dead_code)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, token::Colon};

#[proc_macro]
pub fn analyze(item: TokenStream) -> TokenStream {
    let _: StructWithComments = parse_macro_input!(item);
    quote!().into()
}

#[derive(Debug)]
struct StructWithComments {
    ident: syn::Ident,
    field_name: syn::Ident,
    field_ty: syn::Type,
    outer_attributes: Vec<syn::Attribute>,
    inner_attributes: Vec<syn::Attribute>,
}

impl syn::parse::Parse for StructWithComments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let outer_attributes = input.call(syn::Attribute::parse_outer).unwrap();
        let _: syn::Token![struct] = input.parse().unwrap();
        let ident: syn::Ident = input.parse().unwrap();

        let content;
        let _ = syn::braced!(content in input);
        let inner_attributes = content.call(syn::Attribute::parse_inner).unwrap();
        let field_name: syn::Ident = content.parse().unwrap();
        let _: Colon = content.parse().unwrap();
        let field_ty: syn::Type = content.parse().unwrap();

        Ok(Self {
            ident,
            field_name,
            field_ty,
            outer_attributes,
            inner_attributes,
        })
    }
}
