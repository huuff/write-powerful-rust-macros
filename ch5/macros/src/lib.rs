use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);

    let name = item.ident;
    let vis = item.vis;
    let attrs = item.attrs;

    let fields = match item.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("`private` macro only works on structs with named fields"),
    };

    let private_fields = fields.iter().map(|field| {
        let mut field = field.clone();
        field.vis = syn::Visibility::Inherited;
        quote!(#field)
    });

    let getters = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let method_ident = quote::format_ident!("get_{}", field_ident);
        let field_ty = &field.ty;

        quote!(
            pub fn #method_ident(&self) -> &#field_ty {
                &self.#field_ident
            }
        )
    });

    quote!(
        #(#attrs)*
        #vis struct #name {
            #(#private_fields,)*
        }

        impl #name {
            #(#getters)*
        }
    )
    .into()
}
