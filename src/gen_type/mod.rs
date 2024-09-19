use proc_macro2::TokenStream;

use crate::sparse::Function;

mod in_typing;
mod out_typing;

pub fn gen_out_name(name: String) -> String {
    format!("{}Output", name)
}

pub fn gen_trait_name(name: String) -> String {
    format!("{}Trait", name)
}

pub fn gen_ts(function: Function, body: TokenStream) -> TokenStream {
    let name = function
        .name
        .clone()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .to_string();
    let args = function.args.clone();
    let out_type = out_typing::gen_typing(function.clone());
    if args.is_empty() {
        let out_name = gen_out_name(name.clone());
        quote::quote! {
            type #out_name = #out_type;
            fn #name() -> #out_name {
                #body
            }
        }
    } else {
        let trait_name = gen_trait_name(name.clone());
        let in_types: TokenStream = in_typing::gen_typing(function);
        let mut trimmed_args = args.clone();
        trimmed_args.remove(0);
        let first_arg = args[0].clone();
        quote::quote! {
            trait #trait_name<#(#trimmed_args),*> {
                type Output;
                fn #name(&self, #(#trimmed_args),*) -> Self::Output;
            }
            impl <#(#args),*> #trait_name<#(#trimmed_args),*> for #first_arg
            where
                Self: Clone,
                #in_types
            {
                type Output = #out_type;
                fn #name(&self, #(#trimmed_args),*) -> Self::Output {
                    let #first_arg = self.clone();
                    #body
                }
            }
        }
    }
}
