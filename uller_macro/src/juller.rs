use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Lit, Meta};

pub fn impl_juller(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let output_type = parse_attributes(&ast.attrs);
    let expanded = quote! {
        impl JsonDownload<#output_type> for #name {}
    };
    expanded.into()
}

fn parse_attributes(attrs: &[Attribute]) -> syn::Type {
    attrs
        .iter()
        .find_map(|attr| {
            if attr.path.is_ident("output") {
                if let Ok(Meta::NameValue(meta)) = attr.parse_meta() {
                    if let Lit::Str(lit) = meta.lit {
                        return lit.parse().ok();
                    }
                }
            }
            None
        })
        .expect("No type in output attribute found")
}
