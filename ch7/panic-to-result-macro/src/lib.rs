use proc_macro::TokenStream;
use quote::{quote, ToTokens as _};
use syn::spanned::Spanned as _;

#[proc_macro_attribute]
pub fn panic_to_result(_a: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(item as syn::ItemFn);

    match signature_output_to_result(&ast) {
        Ok(output) => ast.sig.output = output,
        Err(error) => return error.to_compile_error().into(),
    };

    let last_stmt = ast.block.stmts.pop().unwrap();
    ast.block.stmts.push(last_stmt_into_result(last_stmt));

    let new_stmts = ast
        .block
        .stmts
        .into_iter()
        .map(|s| match s {
            syn::Stmt::Expr(e, t) => handle_expression(e, t),
            _ => Ok(s),
        })
        .collect::<Result<Vec<syn::Stmt>, _>>();
    let new_stmts = match new_stmts {
        Ok(it) => it,
        Err(err) => return err.to_compile_error().into(),
    };
    ast.block.stmts = new_stmts;

    ast.to_token_stream().into()
}

/// Convert the output type of the signature to a result
fn signature_output_to_result(ast: &syn::ItemFn) -> Result<syn::ReturnType, syn::Error> {
    let output = match ast.sig.output {
        syn::ReturnType::Default => quote!(-> Result<(), String>),
        syn::ReturnType::Type(_, ref ty) => {
            if ty.to_token_stream().to_string().contains("Result") {
                return Err(syn::Error::new(ast.sig.span(), format!("this macro can only be applied to a function that doesn't return a result. Signature: {}", quote!(#ty))));
            }
            quote!(-> Result<#ty, String>)
        }
    };

    Ok(syn::parse2(output).unwrap())
}

/// Convert the return output type to a result
fn last_stmt_into_result(last_stmt: syn::Stmt) -> syn::Stmt {
    let last_stmt = quote!(Ok(#last_stmt));
    syn::Stmt::Expr(syn::parse2(last_stmt).unwrap(), None)
}

fn handle_expression(
    expression: syn::Expr,
    token: Option<syn::token::Semi>,
) -> Result<syn::Stmt, syn::Error> {
    match expression {
        syn::Expr::If(mut ex_if) => {
            let new_stmts = ex_if
                .then_branch
                .stmts
                .into_iter()
                .map(|s| match s {
                    syn::Stmt::Macro(ref expr_macro) => {
                        let panic_text = extract_panic_content(expr_macro);

                        match panic_text {
                            None => Ok(s),
                            Some(text) if text.is_empty() => Err(syn::Error::new(
                                expr_macro.span(),
                                "please make sure every panic has a message",
                            )),
                            Some(text) => Ok(syn::parse2(quote! {
                                return Err(#text.to_string());
                            })
                            .unwrap()),
                        }
                    }
                    _ => Ok(s),
                })
                .collect::<Result<Vec<_>, _>>();
            ex_if.then_branch.stmts = new_stmts?;
            Ok(syn::Stmt::Expr(syn::Expr::If(ex_if), token))
        }
        _ => Ok(syn::Stmt::Expr(expression, token)),
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
