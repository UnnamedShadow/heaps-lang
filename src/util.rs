pub fn new_ident(name: &String) -> proc_macro2::Ident {
    proc_macro2::Ident::new(name.as_str(), proc_macro2::Span::call_site())
}
