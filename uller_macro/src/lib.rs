extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Attribute, Data, DeriveInput, Fields, Lit, Meta};

///
/// Macros for implement [MakeLink](uller::MakeLink) in query style by using struct as reference
/// ```
/// #[derive(Qller)]
/// #[url = "https://example.com"] // req!
/// struct Pancakes {
///     #[name = "ident"]
///     id: usize,
///     name: String,
///     #[name = "p"]
///     price: f64,
///     description: String,
/// }
///```
#[proc_macro_derive(Qller, attributes(url, name, pos))]
pub fn qller_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("can't parse");
    impl_qller(&ast)
}

fn impl_qller(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let base_url = extract_base_url(ast);

    let mut fields: Vec<_> = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .enumerate()
                .map(|f| {
                    let ident = f.1.ident.as_ref().expect("Parsing ident not found");
                    let rename = extract_rename(&f.1.attrs);
                    let pos = extract_pos(&f.1.attrs).unwrap_or(f.0);

                    (pos, rename, ident.clone())
                })
                .collect(),
            _ => panic!("Qller can only be derived for structs with named fields."),
        },
        _ => panic!("Qller can only be derived for structs."),
    };

    // Сортировка по позиции
    fields.sort_by_key(|(pos, _, _)| *pos);

    // Генерация кода
    let field_names_and_values = fields.into_iter().map(|(_, rename, ident)| {
        let field_name = rename.unwrap_or_else(|| ident.to_string());
        quote! {
            params.push((#field_name.to_string(), self.#ident.to_string()));
        }
    });

    let gen = quote! {
        impl MakeLink for #name {
            fn url_generate(&self) -> url::Url {
                let mut params = Vec::new();
                #(#field_names_and_values)*

                url::Url::parse_with_params(#base_url, &params).expect("Failed to parse URL with params")
            }
        }
    };
    gen.into()
}

fn extract_base_url(ast: &DeriveInput) -> String {
    ast.attrs
        .iter()
        .find_map(|attr| {
            if attr.path.is_ident("url") {
                if let Ok(Meta::NameValue(meta)) = attr.parse_meta() {
                    if let syn::Lit::Str(lit) = meta.lit {
                        return Some(lit.value());
                    }
                }
            }
            None
        })
        .expect("No URL attribute found")
}

fn extract_rename(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path.is_ident("name") {
            if let Ok(Meta::NameValue(meta_name_value)) = attr.parse_meta() {
                if meta_name_value.path.is_ident("name") {
                    if let Lit::Str(lit_str) = &meta_name_value.lit {
                        return Some(lit_str.value());
                    }
                }
            }
        }
    }
    None
}

fn extract_pos(attrs: &[Attribute]) -> Option<usize> {
    for attr in attrs {
        if attr.path.is_ident("pos") {
            if let Ok(Meta::NameValue(meta_name_value)) = attr.parse_meta() {
                if meta_name_value.path.is_ident("pos") {
                    if let Lit::Int(lit_int) = &meta_name_value.lit {
                        return lit_int.base10_parse().ok();
                    }
                }
            }
        }
    }
    None
}
