use proc_macro::TokenStream;
use quote::{quote, ToTokens as _};

#[proc_macro_attribute]
pub fn panic_to_result(_a: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(item as syn::ItemFn);

    ast.sig.output = signature_output_to_result(&ast);

    let last_stmt = ast.block.stmts.pop().unwrap();
    ast.block.stmts.push(last_stmt_into_result(last_stmt));

    let new_stmts: Vec<syn::Stmt> = ast
        .block
        .stmts
        .into_iter()
        .map(|s| match s {
            syn::Stmt::Expr(e, t) => handle_expression(e, t),
            _ => s,
        })
        .collect();
    ast.block.stmts = new_stmts;

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

fn handle_expression(expression: syn::Expr, token: Option<syn::token::Semi>) -> syn::Stmt {
    match expression {
        syn::Expr::If(mut ex_if) => {
            let new_stmts = ex_if
                .then_branch
                .stmts
                .into_iter()
                .map(|s| match s {
                    syn::Stmt::Macro(ref expr_macro) => extract_panic_content(expr_macro)
                        .map(|t| {
                            quote! {
                                return Err(#t.to_string());
                            }
                        })
                        .map(syn::parse2)
                        .map(Result::unwrap)
                        .unwrap_or(s),
                    _ => s,
                })
                .collect::<Vec<_>>();
            ex_if.then_branch.stmts = new_stmts;
            syn::Stmt::Expr(syn::Expr::If(ex_if), token)
        }
        _ => syn::Stmt::Expr(expression, token),
    }
}

fn extract_panic_content(expr_macro: &syn::StmtMacro) -> Option<proc_macro2::TokenStream> {
    let does_panic = expr_macro
        .mac
        .path
        .segments
        .iter()
        .any(|v| v.ident.to_string().eq("panic"));

    if does_panic {
        Some(expr_macro.mac.tokens.clone())
    } else {
        None
    }
}
