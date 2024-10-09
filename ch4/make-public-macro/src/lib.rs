use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed, Ident, Type, Visibility};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields = fields
        .iter()
        .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}

struct StructField {
    name: Ident,
    ty: Type,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

impl Parse for StructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _vis: syn::Result<Visibility> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

        let name = list.first().unwrap().clone();
        let ty = Type::Verbatim(list.last().unwrap().clone().into_token_stream());

        Ok(StructField { name, ty })
    }
}
