use std::collections::HashSet;

use proc_macro2::TokenStream;

use crate::{
    gen_type::{gen_out_name, gen_trait_name},
    sparse::{Function, Statement},
    util::new_ident,
};

#[derive(Clone)]
enum BasicTyping {
    Normal(TokenStream),
    Fn(String),
}

#[derive(Clone)]
enum Typing {
    Normal(BasicTyping),
    FunctionCalls(BasicTyping, Vec<Typing>),
}

fn typing(statement: Statement, args_set: &HashSet<String>) -> Typing {
    match statement {
        Statement::None => Typing::Normal(BasicTyping::Normal(quote::quote! {()})),
        Statement::Literal { .. } => Typing::Normal(BasicTyping::Normal(quote::quote! {String})),
        Statement::VarUsage { name } => {
            if args_set.contains(&name) {
                let name_ident = new_ident(&name);
                Typing::Normal(BasicTyping::Normal(quote::quote! {#name_ident}))
            } else {
                Typing::Normal(BasicTyping::Fn(name))
            }
        }
        Statement::FunctionCall { function, args } => {
            let new_function = typing(*function, args_set);
            let new_args = args.iter().map(|x| typing(x.clone(), args_set));
            match new_function {
                Typing::Normal(f) => Typing::FunctionCalls(f, new_args.collect()),
                Typing::FunctionCalls(..) => {
                    // Typing::FunctionCalls(f, vec![a, new_args.collect()].concat())
                    todo!("higher order functions not supported yet")
                }
            }
        }
    }
}

fn ts(typing: Typing) -> TokenStream {
    match typing {
        Typing::Normal(BasicTyping::Normal(x)) => x,
        Typing::Normal(BasicTyping::Fn(x)) => {
            let x_ident = new_ident(&x);
            quote::quote! {#x_ident}
        }
        Typing::FunctionCalls(f, args) => {
            let args_ts: Vec<_> = args.iter().map(|x| ts(x.clone())).collect();
            match f {
                BasicTyping::Normal(..) => {
                    todo!("higher order functions not supported yet")
                }
                BasicTyping::Fn(x) => {
                    if args.is_empty() {
                        let out_name = new_ident(&gen_out_name(x));
                        quote::quote! {#out_name}
                    } else {
                        let trait_name = new_ident(&gen_trait_name(x));
                        let first_arg = args_ts.first();
                        let args_rest = args_ts[1..].to_vec();
                        quote::quote! {<#first_arg as #trait_name<#(#args_rest),*>>::Output}
                    }
                }
            }
        }
    }
}

pub fn gen_typing(function: Function) -> TokenStream {
    let args_set: HashSet<String> = function.args.iter().cloned().collect();
    let res = typing(function.body.last().unwrap().clone(), &args_set);
    ts(res)
}
