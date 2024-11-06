use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};
use syn::{Data, DataEnum};

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
        Data::Struct(DataStruct {
            fields:
                Fields::Unnamed(FieldsUnnamed {
                    ref mut unnamed, ..
                }),
            ..
        }) => {
            for field in unnamed {
                field.vis = syn::Visibility::Public(Default::default());
            }
        }
        Data::Enum(DataEnum { .. }) => {
            ast.vis = syn::Visibility::Public(Default::default());
        }
        _ => unimplemented!("not available for unions"),
    }

    ast.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let public_version = quote! {};
    public_version.into()
}

#[proc_macro_attribute]
pub fn prefix(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;
    let vis = input.vis;
    let attrs = input.attrs;

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("prefix only works on structs with named fields"),
    };

    let fields = fields.into_iter().map(|field| {
        if field
            .attrs
            .iter()
            .any(|attr| attr.meta.path().is_ident("pfx"))
        {
            let field_attrs = field
                .attrs
                .iter()
                .filter(|attr| !attr.meta.path().is_ident("pfx"));
            let field_name = syn::Ident::new(
                &format!("pfx_{}", field.ident.as_ref().unwrap()),
                proc_macro2::Span::call_site(),
            );
            let field_ty = &field.ty;
            quote!(#(#field_attrs)* #field_name: #field_ty)
        } else {
            quote!(#field)
        }
    });

    quote! {
        #(#attrs)*
        #vis struct #name {
            #(#fields,)*
        }
    }
    .into()
}
