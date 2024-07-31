extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DeriveInput, Fields, Ident, Lit, Meta};

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
    let base_url = extract_base_url(ast);
    let name = &ast.ident;
    let qoute_fields = sort_fielnds(get_fields(ast)).into_iter().map(|(t, v)| {
        quote! {
            params.push((#t, self.#v.to_string()));
        }
    });

    let gen = quote! {
        impl MakeLink for #name {
            fn url_generate(&self) -> url::Url {
                let mut params = Vec::new();
                #(#qoute_fields)*

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

fn get_fields(ast: &DeriveInput) -> Vec<((usize, Option<usize>), String, Ident)> {
    let collected = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                fields
                    .named
                    .iter()
                    .enumerate()
                    .map(|(position, item)| {
                        // field name
                        let ident = item.ident.as_ref().expect("Parsing ident not found");
                        let mut ident_name = ident.to_string();
                        let mut position_perspective: (usize, Option<usize>) = (0, None);
                        // check field attributes
                        item.attrs.iter().for_each(|x| {
                            if let Ok(Meta::NameValue(meta)) = x.parse_meta() {
                                //      name
                                // rename field if possible
                                if meta.path.is_ident("name") {
                                    if let Lit::Str(lit) = &meta.lit {
                                        let lit = lit.value();
                                        if !lit.is_empty() && ident_name != lit {
                                            ident_name = lit
                                        }
                                    }
                                }
                                //      pos
                                // take position
                                if meta.path.is_ident("pos") {
                                    if let Lit::Int(lit) = &meta.lit {
                                        let lit = lit.base10_parse::<usize>().ok();
                                        position_perspective = (position, lit)
                                    }
                                }
                            }
                        });
                        ((position_perspective), ident_name, ident.clone())
                    })
                    .collect::<Vec<((usize, Option<usize>), String, Ident)>>()
            }
            _ => panic!("Only for named data"),
        },
        _ => panic!("It can be use for structures"),
    };
    collected
}

fn sort_fielnds(fields: Vec<((usize, Option<usize>), String, Ident)>) -> Vec<(String, Ident)> {
    let length = fields.len();
    let mut result = vec![None; length];

    let (mut with_custom_pos, without_custom_pos): (Vec<_>, Vec<_>) = fields
        .into_iter()
        .partition(|((_, custom_pos), _, _)| custom_pos.is_some());

    with_custom_pos.sort_by_key(|((_, custom_pos), _, _)| custom_pos.unwrap());

    for ((_, custom_pos), name, ident) in with_custom_pos {
        if let Some(pos) = custom_pos {
            let mut current_pos = pos;
            while current_pos < length && result[current_pos].is_some() {
                current_pos += 1;
            }
            if current_pos < length {
                result[current_pos] = Some((name, ident));
            }
        }
    }

    for ((original_pos, _), name, ident) in without_custom_pos {
        let mut current_pos = original_pos;
        while current_pos < length && result[current_pos].is_some() {
            current_pos += 1;
        }
        if current_pos < length {
            result[current_pos] = Some((name, ident));
        }
    }

    result.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use syn::Ident;

    #[test]
    fn test_sort_elements() {
        let elements = vec![
            (
                (0, None),
                "a".to_string(),
                Ident::new("a", Span::call_site()),
            ),
            (
                (1, None),
                "b".to_string(),
                Ident::new("b", Span::call_site()),
            ),
            (
                (2, None),
                "c".to_string(),
                Ident::new("c", Span::call_site()),
            ),
            (
                (3, Some(2)),
                "d".to_string(),
                Ident::new("d", Span::call_site()),
            ),
            (
                (4, None),
                "e".to_string(),
                Ident::new("e", Span::call_site()),
            ),
            (
                (5, Some(4)),
                "f".to_string(),
                Ident::new("f", Span::call_site()),
            ),
        ];

        let sorted_elements = sort_fielnds(elements);

        let expected = vec![
            ("a".to_string(), Ident::new("a", Span::call_site())),
            ("b".to_string(), Ident::new("b", Span::call_site())),
            ("d".to_string(), Ident::new("d", Span::call_site())),
            ("c".to_string(), Ident::new("c", Span::call_site())),
            ("f".to_string(), Ident::new("f", Span::call_site())),
            ("e".to_string(), Ident::new("e", Span::call_site())),
        ];

        assert_eq!(sorted_elements, expected);
    }
}
