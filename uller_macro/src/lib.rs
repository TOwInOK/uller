use proc_macro::TokenStream;
use qller::impl_qller;
use syn::{parse_macro_input, DeriveInput};

mod buller;
mod juller;
mod qller;
#[proc_macro_derive(Qller, attributes(url, name, pos))]
pub fn qller_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_qller(&ast)
}

#[cfg(feature = "juller")]
#[proc_macro_derive(Juller, attributes(output))]
pub fn juller_derive(input: TokenStream) -> TokenStream {
    use juller::impl_juller;

    let ast = parse_macro_input!(input as DeriveInput);
    impl_juller(&ast)
}

#[cfg(feature = "buller")]
#[proc_macro_derive(Buller, attributes())]
pub fn buller_derive(input: TokenStream) -> TokenStream {
    use buller::impl_buller;

    let ast = parse_macro_input!(input as DeriveInput);
    impl_buller(&ast)
}
