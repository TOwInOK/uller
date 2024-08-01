//! This crate provide useful interface for creating links
//! and
//! download data by created links

#[cfg(feature = "macro")]
pub use uller_macro::*;
pub use url::Url;

// for Qller
/// Trait for unification [Url] builder
pub trait MakeLink {
    fn url_generate(&self) -> crate::Url;
}
#[cfg(feature = "juller")]
// for Jller
/// Desirialize to json any struct by [link](MakeLink)
#[async_trait::async_trait]
pub trait JsonDownload<T>: MakeLink
where
    T: for<'a> serde::Deserialize<'a>,
{
    /// Make url and parse to `<T>`
    async fn download(&self) -> Result<T, Box<dyn std::error::Error>> {
        Ok(reqwest::get(self.url_generate()).await?.json().await?)
    }
    /// Make url and parse to `<T>` with url anotate
    async fn download_verbose(&self) -> Result<T, Box<dyn std::error::Error>> {
        let url = self.url_generate();
        println!("{:#?}", String::from(url.clone()));
        Ok(reqwest::get(url).await?.json().await?)
    }
}

#[cfg(feature = "buller")]
// for Bller
/// Download bytes by [link](MakeLink)
#[async_trait::async_trait]
pub trait BytesDownload: MakeLink {
    /// Make url and parse to Bytes
    async fn download(&self) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
        Ok(reqwest::get(self.url_generate()).await?.bytes().await?)
    }
    /// Make url and parse to Bytes with url anotate
    async fn download_verbose(&self) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
        let url = self.url_generate();
        println!("{:#?}", String::from(url.clone()));
        Ok(reqwest::get(url).await?.bytes().await?)
    }
}

/// Just all that you need.
pub mod prelude {
    #[cfg(feature = "buller")]
    pub use crate::BytesDownload;
    #[cfg(feature = "juller")]
    pub use crate::JsonDownload;

    pub use crate::MakeLink;
    pub use crate::Url;

    #[cfg(feature = "buller")]
    pub use uller_macro::Buller;
    #[cfg(feature = "juller")]
    pub use uller_macro::Juller;
    #[cfg(feature = "macro")]
    pub use uller_macro::Qller;
}
