use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;

use crate::{
    gen_type::{gen_out_name, gen_trait_name},
    sparse::{Function, Statement},
};

#[derive(Clone)]
enum Typing {
    None,
    Static(TokenStream),
    Trait(Box<TraitTyping>),
}

#[derive(Clone)]
struct TraitTyping {
    self_type: Typing,
    name: String,
    types: Vec<Typing>,
}

fn typing(statement: Statement, args_set: &HashSet<String>) -> Typing {
    match statement {
        Statement::None => Typing::None,
        Statement::Literal { .. } => Typing::Static(quote::quote! {String}),
        Statement::VarUsage { name } => {
            if args_set.contains(&name) {
                let name_ident =
                    proc_macro2::Ident::new(name.as_str(), proc_macro2::Span::call_site());
                Typing::Static(quote::quote! {#name_ident})
            } else {
                todo!("higher order functions not supported yet")
            }
        }
        Statement::FunctionCall { function, args } => {
            let new_args: Vec<_> = args.iter().map(|x| typing(x.clone(), args_set)).collect();
            match *function {
                Statement::None => panic!("function call to None"),
                Statement::Literal { .. } => panic!("can't call a literal"),
                Statement::VarUsage { name } => {
                    if args_set.contains(&name) {
                        todo!("higher order functions not supported yet")
                    } else if new_args.is_empty() {
                        let out_name = gen_out_name(name.clone());
                        let out_name_ident = proc_macro2::Ident::new(
                            out_name.as_str(),
                            proc_macro2::Span::call_site(),
                        );
                        Typing::Static(quote::quote! {#out_name_ident})
                    } else {
                        let trait_name = gen_trait_name(name.clone());
                        Typing::Trait(Box::new(TraitTyping {
                            self_type: new_args[0].clone(),
                            name: trait_name,
                            types: new_args[1..].to_vec(),
                        }))
                    }
                }
                Statement::FunctionCall { .. } => {
                    todo!("higher order functions not supported yet")
                }
            }
        }
    }
}

fn ts(typing: Typing, acc: &mut TokenStream) -> TokenStream {
    match typing {
        Typing::None => quote::quote! {()},
        Typing::Static(token_stream) => token_stream,
        Typing::Trait(trait_typing) => {
            let self_type = ts(trait_typing.self_type, acc);
            let trait_name =
                proc_macro2::Ident::new(trait_typing.name.as_str(), proc_macro2::Span::call_site());
            let types: Vec<_> = trait_typing
                .types
                .iter()
                .map(|x| ts(x.clone(), acc))
                .collect();
            let self_type_clone = self_type.clone();
            let trait_name_clone = trait_name.clone();
            let types_clone = types.clone();
            acc.append_all(quote::quote! {#self_type_clone:#trait_name_clone<#(#types_clone),*>,});
            quote::quote! {<#self_type as #trait_name<#(#types),*>>::Output}
        }
    }
}

pub fn gen_typing(function: &Function) -> TokenStream {
    let args_set: HashSet<String> = function.args.iter().cloned().collect();
    let mut acc = TokenStream::new();
    function.body.iter().for_each(|line| {
        ts(typing(line.clone(), &args_set.clone()), &mut acc);
    });
    acc
}
