//! This crate provide useful interface for creating links
//! and
//! download data by created links

/// This macro crate give you
/// usefull autogeneraing implimentations for structures
pub use uller_macro::*;
use url::Url;

// for Qller
/// Trait for unification [Url] builder
pub trait MakeLink {
    fn url_generate(&self) -> Url;
}
#[cfg(feature = "juller")]
// for Jller
/// Desirialize to json any struct by [link](MakeLink)
#[async_trait::async_trait]
pub trait JsonDownload<T>: MakeLink
where
    T: for<'a> serde::Deserialize<'a>,
{
    async fn download(&self) -> Result<T, Box<dyn std::error::Error>> {
        Ok(reqwest::get(self.url_generate()).await?.json().await?)
    }
}

#[cfg(feature = "buller")]
// for Bller
/// Download [bytes](bytes:Bytes) by [link](MakeLink)
#[async_trait::async_trait]
pub trait BytesDownload: MakeLink {
    async fn download(&self) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
        Ok(reqwest::get(self.url_generate()).await?.bytes().await?)
    }
}
