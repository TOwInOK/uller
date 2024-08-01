use proc_macro::TokenStream;
use qller::impl_qller;
use syn::{parse_macro_input, DeriveInput};

mod qller;
/// Macros for implement [MakeLink](uller::MakeLink) in query style by using struct as prompt
/// ### Example
/// ```
/// use uller::prelude;
///
/// #[derive(Qller)]
/// #[url = "http://127.0.0.1:1234/"]
/// struct Test {
///     #[name = "f"] // rename to "f"
///     f111: String,
///     #[name = "v"] // rename to "v"
///     #[pos = 0] // move it in first position
///     v222: String
/// }
/// ```
/// will convert to http://127.0.0.1:1234/?v={value}&f={value}
///
/// note: position starts with 0 pos like an array.
#[proc_macro_derive(Qller, attributes(url, name, pos))]
pub fn qller_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_qller(&ast)
}

#[cfg(feature = "buller")]
mod buller;

#[cfg(feature = "juller")]
mod juller;

#[cfg(feature = "juller")]
/// Macros for download `<T>` using struct which implement [MakeLink] ([Qller]) and [JsonDownload]
/// ### Example
/// ```
/// use uller::prelude;
///
/// #[derive(Qller, Juller)]
/// #[output = "TestOut"]
/// #[url = "http://127.0.0.1:41112/"]
/// struct Test {
///     f: String,
///     v: String,
/// }
///
/// #[derive(Deserialize, Debug)]
/// struct TestOut {
///     field: String,
/// }
///
/// async fn convert(st: &Test) -> TestOut {
///     st.download().await.unwrap()
///     // or
///     st.download_verbose().await.unwrap()
/// }
/// ```
#[proc_macro_derive(Juller, attributes(output))]
pub fn juller_derive(input: TokenStream) -> TokenStream {
    use juller::impl_juller;

    let ast = parse_macro_input!(input as DeriveInput);
    impl_juller(&ast)
}

#[cfg(feature = "buller")]
/// Macros for download [Bytes] using struct which implement [MakeLink] ([Qller]) and [BytesDownload]
/// ### Example
/// ```
/// use uller::prelude;
///
/// #[derive(Qller, Buller)]
/// #[url = "http://127.0.0.1:41112/"]
/// struct Test {
///     f: String,
///     v: String,
/// }
///
/// async fn convert(st: &Test) -> bytes::Bytes {
///     st.download().await.unwrap()
///     // or
///     st.download_verbose().await.unwrap()
/// }
/// ```
#[proc_macro_derive(Buller, attributes())]
pub fn buller_derive(input: TokenStream) -> TokenStream {
    use buller::impl_buller;

    let ast = parse_macro_input!(input as DeriveInput);
    impl_buller(&ast)
}
