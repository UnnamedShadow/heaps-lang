use proc_macro::TokenStream;
use quote::TokenStreamExt;
use sparse::sparse;

mod gen_body_sync;
mod gen_type;
mod sparse;
mod types;
// mod types_test;

#[proc_macro]
pub fn heaps_sync(input: TokenStream) -> TokenStream {
    let functions = sparse(input);
    let mut ts = proc_macro2::TokenStream::new();
    for function in functions {
        ts.append_all(gen_type::gen_ts(
            function.clone(),
            gen_body_sync::gen_ts(function),
        ));
    }
    println!("{}", ts.to_string());
    ts.into()
}
