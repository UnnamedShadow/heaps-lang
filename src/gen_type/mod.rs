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
    let name_ident = proc_macro2::Ident::new(name.as_str(), proc_macro2::Span::call_site());
    if args.is_empty() {
        let out_name = gen_out_name(name.clone());
        let out_name_ident =
            proc_macro2::Ident::new(out_name.as_str(), proc_macro2::Span::call_site());
        quote::quote! {
            type #out_name_ident = #out_type;
            fn #name_ident() -> #out_name_ident {
                #body
            }
        }
    } else {
        let args_ident: Vec<_> = args
            .iter()
            .map(|x| proc_macro2::Ident::new(x.as_str(), proc_macro2::Span::call_site()))
            .collect();
        let trait_name = gen_trait_name(name.clone());
        let in_types = in_typing::gen_typing(function);
        let mut trimmed_args = args_ident.clone();
        trimmed_args.remove(0);
        let first_arg = args_ident[0].clone();
        let trait_name_ident =
            proc_macro2::Ident::new(trait_name.as_str(), proc_macro2::Span::call_site());
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
