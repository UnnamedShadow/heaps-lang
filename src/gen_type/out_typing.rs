use std::collections::HashSet;

use proc_macro2::TokenStream;

use crate::{
    gen_type::gen_out_name,
    sparse::{Function, Statement},
};

use super::gen_trait_name;

#[derive(Clone)]
enum BasicTyping {
    Normal(TokenStream),
    Fn(String),
}

#[derive(Clone)]
enum Typing {
    Normal(BasicTyping),
    FunctionCalls(BasicTyping, Vec<Box<Typing>>),
}

fn typing(statement: Statement, args_set: HashSet<String>) -> Typing {
    match statement {
        Statement::None => Typing::Normal(BasicTyping::Normal(quote::quote! {()})),
        Statement::Literal { content } => Typing::Normal(BasicTyping::Normal(quote::quote! {&str})),
        Statement::VarUsage { name } => {
            if args_set.contains(&name) {
                Typing::Normal(BasicTyping::Normal(quote::quote! {#name}))
            } else {
                Typing::Normal(BasicTyping::Fn(name))
            }
        }
        Statement::FunctionCall { function, args } => {
            let new_function = typing(*function, args_set.clone());
            let new_args = args
                .iter()
                .map(|x| Box::new(typing(*(x.clone()), args_set.clone())));
            match new_function {
                Typing::Normal(f) => Typing::FunctionCalls(f, new_args.collect()),
                Typing::FunctionCalls(f, a) => {
                    Typing::FunctionCalls(f, vec![a, new_args.collect()].concat())
                }
            }
        }
    }
}

fn ts(typing: Typing) -> TokenStream {
    match typing {
        Typing::Normal(BasicTyping::Normal(x)) => x,
        Typing::Normal(BasicTyping::Fn(x)) => {
            todo!("sorry idk if ill implement it")
        }
        Typing::FunctionCalls(f, args) => {
            let args_ts = args.iter().map(|x| ts(*x.clone()));
            match f {
                BasicTyping::Fn(x) => {
                    let fun = gen_out_name(x);
                    quote::quote! {#fun<#(#args_ts),*>}
                }
                BasicTyping::Normal(x) => quote::quote! {#x<#(#args_ts),*>},
            }
        }
    }
}

pub fn gen_typing(function: Function) -> TokenStream {
    let args_set: HashSet<String> = function.args.iter().cloned().collect();
    let res = typing(function.body.last().unwrap().clone(), args_set);
    ts(res)
}
