use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;

use crate::{
    gen_type::gen_trait_name,
    sparse::{Function, Statement},
};

fn typing(statement: Statement, args_set: HashSet<String>) -> TokenStream {
    todo!()
}

pub fn gen_typing(function: Function) -> TokenStream {
    let args_set: HashSet<String> = function.args.iter().cloned().collect();
    let mut ts = TokenStream::new();
    for line in function.body {
        ts.append_all(typing(line, args_set.clone()));
    }
    ts
}
