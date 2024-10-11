use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::token::Colon;
use syn::{
    parse_macro_input, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Type,
    Visibility,
};
use syn::{Data, DataEnum};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            let builder_fields = named
                .iter()
                .map(|f| syn::parse2::<NamedStructField>(f.to_token_stream()).unwrap());

            quote! {
               pub struct #name {
                   #(#builder_fields,)*
               }
            }
            .into()
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            let builder_fields = unnamed
                .iter()
                .map(|f| syn::parse2::<UnnamedStructField>(f.to_token_stream()).unwrap());

            quote! {
                pub struct #name(#(#builder_fields,)*);
            }
            .into()
        }
        Data::Enum(DataEnum { ref variants, .. }) => {
            let variants_iter = variants.into_iter();
            quote! {
                pub enum #name {
                    #(#variants_iter,)*
                }
            }
            .into()
        }
        _ => unimplemented!("not available for unions"),
    }
}

struct NamedStructField {
    name: Ident,
    ty: Type,
}

impl ToTokens for NamedStructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

impl Parse for NamedStructField {
    // punctuated
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _vis: syn::Result<Visibility> = input.parse();
        let name = input.parse::<Ident>()?;
        let _ = input.parse::<Colon>()?;
        let ty = input.parse::<Type>()?;

        Ok(NamedStructField { name, ty })
    }
}

struct UnnamedStructField(Type);

impl ToTokens for UnnamedStructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = &self.0;
        quote!(pub #ty).to_tokens(tokens)
    }
}

impl Parse for UnnamedStructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _vis: syn::Result<Visibility> = input.parse();
        let ty: syn::Type = input.parse()?;

        Ok(UnnamedStructField(ty))
    }
}

#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let public_version = quote! {};
    public_version.into()
}

// #[proc_macro_attribute]
// pub fn add_suffix(_attr: TokenStream, item: TokenStream) -> TokenStream {}
