use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, Parser as _};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Data;
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_attribute]
pub fn public(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    let excluded_fields = parse_macro_input!(attr as ExcludedFields);

    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref mut named, .. }),
            ..
        }) => {
            for field in named {
                if !excluded_fields.matches_ident(&field.ident) {
                    field.vis = syn::Visibility::Public(Default::default());
                }
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

const EXCLUDE_ATTR_NAME: &str = "exclude";

#[derive(Default)]
struct ExcludedFields {
    fields: Vec<String>,
}

impl ExcludedFields {
    fn matches_ident(&self, name: &Option<syn::Ident>) -> bool {
        name.as_ref()
            .map(|n| n.to_string())
            .map(|n| self.fields.iter().any(|f| *f == n))
            .unwrap_or(false)
    }
}

impl Parse for ExcludedFields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.parse::<syn::MetaList>() {
            Ok(meta_list) => {
                if meta_list
                    .path
                    .segments
                    .iter()
                    .any(|s| s.ident == EXCLUDE_ATTR_NAME)
                {
                    let parser = Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated;
                    let identifiers = parser.parse2(meta_list.clone().tokens).unwrap();

                    let fields = identifiers.iter().map(|v| v.to_string()).collect();
                    Ok(ExcludedFields { fields })
                } else {
                    Ok(ExcludedFields::default())
                }
            }
            _ => Ok(ExcludedFields::default()),
        }
    }
}
