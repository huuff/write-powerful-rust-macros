use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello, World!");
            }
        }
    };
    add_hello_world.into()
}

// TODO this breaks!
#[proc_macro_derive(HelloNoDep)]
pub fn hello_no_dep(item: TokenStream) -> TokenStream {
    fn ident_name(item: TokenTree) -> String {
        match item {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("no ident"),
        }
    }

    let name = ident_name(item.into_iter().nth(1).unwrap());

    format!(
        r#"
          impl {name} {{
            fn hello_world(&self) {{
              println!("Hello, World")
            }}
          }} 
        "#
    )
    .parse()
    .unwrap()
}
