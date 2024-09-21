use proc_macro2::TokenStream;

use crate::{sparse::Function, util::new_ident};

mod in_typing;
mod out_typing;

pub fn gen_out_name(name: String) -> String {
    format!("{}Output", name)
}

pub fn gen_trait_name(name: String) -> String {
    format!("{}Trait", name)
}

pub fn gen_ts(function: &Function, body: TokenStream) -> TokenStream {
    let Function {
        name: original_name,
        args,
        ..
    } = function;
    let name = original_name
        .trim_start_matches('(')
        .trim_end_matches(')')
        .to_string();
    let out_type = out_typing::gen_typing(function.clone());
    let name_ident = new_ident(&name);
    if args.is_empty() {
        let out_name = gen_out_name(name);
        let out_name_ident = new_ident(&out_name);
        quote::quote! {
            type #out_name_ident = #out_type;
            fn #name_ident() -> #out_name_ident {
                #body
            }
        }
    } else {
        let args_ident: Vec<_> = args.iter().map(new_ident).collect();
        let trait_name = gen_trait_name(name);
        let in_types = in_typing::gen_typing(function);
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
                    #body
                }
            }
        }
    }
}
