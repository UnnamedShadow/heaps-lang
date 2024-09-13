use proc_macro2::TokenStream;
use quote::TokenStreamExt;

use crate::sparse::{Function, Statement};

fn gen_statement(statement: Statement) -> TokenStream {
    match statement {
        Statement::None => quote::quote! {},
        Statement::VarUsage { name } => quote::quote! {#name},
        Statement::FunctionCall { function, args } => {
            let new_function = gen_statement(*function);
            let new_args = args.iter().map(|x| gen_statement(*(x.clone())));
            quote::quote! {#new_function(#(#new_args),*)}
        }
        Statement::Literal { content } => {
            let lit = proc_macro2::Literal::string(content.as_str());
            quote::quote! {#lit}
        }
    }
}

pub fn gen_ts(function: Function) -> TokenStream {
    let mut ts = TokenStream::new();
    for line in function.body {
        let line_ts = gen_statement(line);
        ts.append_all(quote::quote! {#line_ts;})
    }
    ts
}
