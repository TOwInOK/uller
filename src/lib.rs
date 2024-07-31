//! This crate provide useful interface for creating links
//! and
//! download data by created links

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::error::Error;
use url::Url;

/// This macro crate give you
/// usefull autogeneraing implimentations for structures
pub use uller_macro::*;

// for Qller
/// Trait for unification [Url] builder
pub trait MakeLink {
    fn url_generate(&self) -> Url;
}

// for Jller
/// Desirialize to json any struct by [link](MakeLink)
#[async_trait]
pub trait JsonDownload<T>: MakeLink
where
    T: DeserializeOwned,
{
    async fn download(&self) -> Result<T, Box<dyn Error>> {
        Ok(reqwest::get(self.url_generate()).await?.json().await?)
    }
}

// for Bller
/// Download [bytes](bytes:Bytes) by [link](MakeLink)
#[async_trait]
pub trait BytesDownload: MakeLink {
    async fn download(&self) -> Result<bytes::Bytes, Box<dyn Error>> {
        Ok(reqwest::get(self.url_generate()).await?.bytes().await?)
    }
}
