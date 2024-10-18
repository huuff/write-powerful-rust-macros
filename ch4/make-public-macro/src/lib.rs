use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::{
    parse_macro_input, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Token, Type,
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
            let builder_fields = named.iter().map(|f| {
                let mut nu_field = f.clone();
                nu_field.vis = syn::Visibility::Public(Token![pub](proc_macro2::Span::call_site()));
                quote!(#nu_field)
            });

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
