use proc_macro::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::Data;
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);

    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref mut named, .. }),
            ..
        }) => {
            for field in named {
                field.vis = syn::Visibility::Public(Default::default());
            }
        }
        _ => {
            return syn::Error::new(ast.span(), "only works on structs with named fields")
                .to_compile_error()
                .into()
        }
    }

    ast.into_token_stream().into()
}
