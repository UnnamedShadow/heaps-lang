use std::collections::HashMap;

use proc_macro2::TokenStream;

use crate::sparse::{Function, Statement};

fn out_name(name: String) -> String {
    format!("{}Output", name)
}

#[derive(Debug, Clone)]
struct TypeDef {}

impl TypeDef {
    fn output(&self) -> TokenStream {
        todo!()
    }
}

fn gen_types(function: Function) -> TypeDef {
    todo!()
}

fn gen_statement_types(statement: Statement) -> TypeDef {
    todo!()
}

pub fn gen_ts(function: Function, body: TokenStream) -> TokenStream {
    let name = function
        .name
        .clone()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .to_string();
    let ts;
    let types = gen_types(function.clone());
    let output = types.output();
    if function.args.is_empty() {
        let out_name = out_name(name.clone());
        ts = quote::quote! {
            type #out_name = #output;
            fn #name () -> #out_name {
                #body
            }
        };
    } else {
        let first_arg = function.args[0].clone();
        ts = if function.args.len() == 1 {
            quote::quote! {
                trait #name {
                    type Output;
                    // TODO
                    fn #name(self) -> Self::Output;
                }

                impl <#first_arg> #name for #first_arg
                where
                // TODO
                {
                    type Output = #output;
                    fn #name(self) -> Self::Output {
                        #body
                    }
                }
            }
        } else {
            quote::quote! {
                trait #name {
                    type Output;
                    // TODO
                    fn #name(self, todo) -> Self::Output;
                }

                impl <#first_arg> #name for #first_arg
                // TODO
                where
                {
                    type Output = #output; //??
                    fn #name(self, todo) -> Self::Output {
                        #body
                    }
                }
            }
        }
    }
    ts
}
