use proc_macro::TokenStream;
use quote::{quote, ToTokens as _};

#[proc_macro_attribute]
pub fn panic_to_result(_a: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(item as syn::ItemFn);

    ast.sig.output = signature_output_to_result(&ast);
    let last_stmt = ast.block.stmts.pop().unwrap();
    ast.block.stmts.push(last_stmt_into_result(last_stmt));

    ast.to_token_stream().into()
}

/// Convert the output type of the signature to a result
fn signature_output_to_result(ast: &syn::ItemFn) -> syn::ReturnType {
    let output = match ast.sig.output {
        syn::ReturnType::Default => quote!(-> Result<(), String>),
        syn::ReturnType::Type(_, ref ty) => quote!(-> Result<#ty, String>),
    };

    syn::parse2(output).unwrap()
}

/// Convert the return output type to a result
fn last_stmt_into_result(last_stmt: syn::Stmt) -> syn::Stmt {
    let last_stmt = quote!(Ok(#last_stmt));
    syn::Stmt::Expr(syn::parse2(last_stmt).unwrap(), None)
}
