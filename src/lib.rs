use proc_macro::TokenStream;
use quote::TokenStreamExt;

mod gen_body_sync;
mod gen_type;
mod sparse;
mod util;

#[proc_macro]
pub fn heaps_sync(input: TokenStream) -> TokenStream {
    let functions = sparse::sparse(input);
    let mut ts = proc_macro2::TokenStream::new();
    for function in functions {
        let body = gen_body_sync::gen_ts(&function);
        ts.append_all(gen_type::gen_ts(&function, body));
    }
    println!("{}", ts);
    ts.into()
}
