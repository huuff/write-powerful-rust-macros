use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, spanned::Spanned as _, token::Comma, Field, Ident, Type};

pub fn builder_field_definitions(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, ty) = get_name_and_type(f);
        quote!(#name: Option<#ty>)
    })
}

pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
    use_defaults: bool,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(move |f| {
        let field_name = &f.ident;
        let field_name_as_string = field_name.as_ref().unwrap().to_string();

        let handle_missing = if use_defaults {
            quote! {
                unwrap_or_default()
            }
        } else {
            quote! {
                expect(concat!("field not set: ", #field_name_as_string))
            }
        };

        quote! {
            #field_name: self.#field_name.#handle_missing
        }
    })
}

pub fn builder_field_init(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        quote!(#field_name: None)
    })
}

pub fn builder_methods(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_ty) = get_name_and_type(f);
        let rename = extract_attribute_from_field(f, "rename")
            .map(|a| &a.meta)
            .map(|m| {
                // let nested = m.require_list().unwrap();
                // let a: syn::LitStr = nested.parse_args().unwrap();
                // Ident::new(&a.value(), a.span())
                let nested = m.require_name_value().unwrap();
                match nested {
                    syn::MetaNameValue {
                        value:
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(literal_string),
                                ..
                            }),
                        ..
                    } => Ident::new(&literal_string.value(), literal_string.span()),
                    _ => panic!("expected a lit rename value"),
                }
            });

        let setter_name = if let Some(ref rename) = rename {
            rename
        } else {
            field_name.as_ref().unwrap()
        };

        quote! {
            pub fn #setter_name(mut self, input: #field_ty) -> Self {
                self.#field_name = Some(input);
                self
            }
        }
    })
}

pub fn optional_default_asserts(fields: &Punctuated<syn::Field, Comma>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let name = &f.ident.as_ref().unwrap();
            let ty = &f.ty;
            let assertion_ident = format_ident!("__{}DefaultAssertion", name);

            quote::quote_spanned! { ty.span() =>
                    #[allow(non_camel_case_types)]
                    struct #assertion_ident where #ty: core::default::Default;
            }
        })
        .collect()
}

fn get_name_and_type(f: &Field) -> (&Option<Ident>, &Type) {
    let field_name = &f.ident;
    let field_ty = &f.ty;
    (field_name, field_ty)
}

fn extract_attribute_from_field<'a>(f: &'a Field, name: &'a str) -> Option<&'a syn::Attribute> {
    f.attrs.iter().find(|&attr| attr.path().is_ident(name))
}
