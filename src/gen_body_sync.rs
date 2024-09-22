use proc_macro2::TokenStream;
use quote::TokenStreamExt;

use crate::{
    sparse::{Function, Statement},
    util::new_ident,
};

fn gen_statement(statement: &Statement) -> TokenStream {
    match statement {
        Statement::None => quote::quote! {},
        Statement::VarUsage { name } => {
            let name_ident = new_ident(name);
            quote::quote! {#name_ident}
        }
        Statement::FunctionCall { function, args } => {
            let new_function = gen_statement(function);
            let mut new_args = args.iter().map(gen_statement);
            if let Some(first_arg) = new_args.next() {
                quote::quote! {#first_arg.#new_function(#(#new_args),*)}
            } else {
                quote::quote! {#new_function()}
            }
        }
        Statement::Literal { content } => {
            quote::quote! {heaps_std::str::Str(#content.to_string())}
        }
    }
}

pub fn gen_ts(function: &Function) -> TokenStream {
    let mut ts = TokenStream::new();
    function.body.iter().for_each(|line| {
        let line_ts = gen_statement(line);
        ts.append_all(quote::quote! {; #line_ts});
    });
    ts
}
