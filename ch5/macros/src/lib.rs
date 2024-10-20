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

    let constructor = {
        let field_ty_pairs = fields.iter().map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote!(#ident: #ty)
        });
        let idents_only = fields.iter().map(|f| &f.ident);

        quote!(
            fn new(#(#field_ty_pairs,)*) -> Self {
                Self {
                    #(#idents_only,)*
                }
            }
        )
    };

    quote!(
        #(#attrs)*
        #vis struct #name {
            #(#private_fields,)*
        }

        impl #name {
            #constructor
            #(#getters)*
        }
    )
    .into()
}

#[proc_macro]
pub fn compose(item: TokenStream) -> TokenStream {
    let ci: ComposeInput = parse_macro_input!(item);

    quote!({
        fn compose_two<FIRST, SECOND, THIRD, F, G>(first: F, second: G) -> impl Fn(FIRST) -> THIRD
        where
            F: Fn(FIRST) -> SECOND,
            G: Fn(SECOND) -> THIRD,
        {
            move |x| second(first(x))
        }
        #ci
    })
    .into()
}

struct ComposeInput {
    expressions: syn::punctuated::Punctuated<syn::Ident, syn::Token!(>>)>,
}

impl syn::parse::Parse for ComposeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expressions: syn::punctuated::Punctuated::parse_terminated(input).unwrap(),
        })
    }
}

impl quote::ToTokens for ComposeInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut total = None;
        let mut as_idents: Vec<&syn::Ident> = self.expressions.iter().collect();
        let last_ident = as_idents.pop().unwrap();

        as_idents.iter().rev().for_each(|i| {
            if let Some(current_total) = &total {
                total = Some(quote!(compose_two(#i, #current_total)))
            } else {
                total = Some(quote!(compose_two(#i, #last_ident)))
            }
        });
        total.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as syn::Ident);

    quote!(
        impl #ident {
            pub fn hello_world(&self) {
                println!("Hello, World!")
            }
        }
    )
    .into()
}
