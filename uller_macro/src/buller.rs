use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_buller(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    quote! {
       impl BytesDownload for #name {}
    }
    .into()
}
