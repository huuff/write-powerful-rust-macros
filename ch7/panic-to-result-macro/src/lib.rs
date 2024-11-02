use proc_macro::TokenStream;
use quote::{quote, ToTokens as _};

#[proc_macro_attribute]
pub fn panic_to_result(_a: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(item as syn::ItemFn);

    let output = match ast.sig.output {
        syn::ReturnType::Default => quote!(-> Result<(), String>),
        syn::ReturnType::Type(_, ref ty) => quote!(-> Result<#ty, String>),
    };

    ast.sig.output = syn::parse2(output).unwrap();

    let last = ast.block.stmts.pop().unwrap();
    let last = quote!(Ok(#last));
    let last = syn::Stmt::Expr(syn::parse2(last).unwrap(), None);

    ast.block.stmts.push(last);

    ast.to_token_stream().into()
}
