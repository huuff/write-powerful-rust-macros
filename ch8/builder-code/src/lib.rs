mod fields;
mod utils;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use fields::{
    builder_definition, builder_impl_for_struct, builder_methods, marker_trait_and_structs,
};

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();

    let name = ast.ident;

    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only implemented for structs with named fields"),
    };

    let builder = builder_definition(&name, fields);
    let builder_method_for_struct = builder_impl_for_struct(&name, fields);

    let marker_and_structs = marker_trait_and_structs(&name, fields);
    let builder_methods = builder_methods(&name, fields);

    quote! {
        #builder
        #builder_method_for_struct
        #marker_and_structs
        #builder_methods
    }
}
