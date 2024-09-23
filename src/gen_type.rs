use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;

use crate::{
    sparse::{Function, Statement},
    util::new_ident,
};

pub fn gen_out_name(name: String) -> String {
    format!("{}Output", name)
}

pub fn gen_trait_name(name: String) -> String {
    format!("{}Trait", name)
}

fn gen_statement_ts(
    statement: Statement,
    acc: &mut TokenStream,
    args_set: &HashSet<String>,
) -> TokenStream {
    match statement {
        Statement::None => quote::quote! {()},
        Statement::VarUsage { name } => {
            let name_ident = new_ident(&{
                if args_set.contains(&name) {
                    name
                } else {
                    gen_trait_name(name)
                }
            });
            quote::quote! {#name_ident}
        }
        Statement::Literal { .. } => quote::quote! {heaps_std::str::Str},
        Statement::FunctionCall { function, args } => {
            let new_function = gen_statement_ts(*function, acc, args_set);
            let new_args: Vec<_> = args
                .iter()
                .map(|x| gen_statement_ts(x.clone(), acc, args_set))
                .collect();
            let first_arg = new_args.first();
            let args_rest = new_args[1..].to_vec();
            acc.append_all(quote::quote! {#first_arg: #new_function<#(#args_rest),*>,});
            quote::quote! {<#first_arg as #new_function<#(#args_rest),*>>::Output}
        }
    }
}

pub fn gen_ts(function: &Function, body_ts: TokenStream) -> TokenStream {
    let Function {
        name: original_name,
        args,
        body,
    } = function;
    let name = original_name
        .trim_start_matches('(')
        .trim_end_matches(')')
        .to_string();
    let mut in_types = TokenStream::new();
    let args_set: HashSet<String> = args.iter().cloned().collect();
    for line in body[..body.len() - 1].iter() {
        gen_statement_ts(line.clone(), &mut in_types, &args_set);
    }
    let out_type = gen_statement_ts(body.last().unwrap().clone(), &mut in_types, &args_set);
    let name_ident = new_ident(&name);
    if args.is_empty() {
        let out_name = gen_out_name(name);
        let out_name_ident = new_ident(&out_name);
        quote::quote! {
            type #out_name_ident = #out_type;
            fn #name_ident() -> #out_name_ident {
                #body_ts
            }
        }
    } else {
        let args_ident: Vec<_> = args.iter().map(new_ident).collect();
        let trait_name = gen_trait_name(name);
        let mut trimmed_args = args_ident.clone();
        trimmed_args.remove(0);
        let first_arg = args_ident[0].clone();
        let trait_name_ident = new_ident(&trait_name);
        quote::quote! {
            trait #trait_name_ident<#(#trimmed_args),*> {
                type Output;
                fn #name_ident(&self, #(#trimmed_args: #trimmed_args),*) -> Self::Output;
            }
            impl <#(#args_ident),*> #trait_name_ident<#(#trimmed_args),*> for #first_arg
            where
                Self: Clone,
                #in_types
            {
                type Output = #out_type;
                fn #name_ident(&self, #(#trimmed_args: #trimmed_args),*) -> Self::Output {
                    let #first_arg = self.clone()
                    #body_ts
                }
            }
        }
    }
}
