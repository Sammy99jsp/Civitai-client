use std::{collections::HashMap, fs};

use lazy_static::lazy_static;
use reqwest::{Client, IntoUrl, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize};
use serde_aux::field_attributes::deserialize_number_from_string;

pub mod endpoints;
pub mod paginated;
pub mod types;
pub mod utils;

pub use paginated::{PageIterator, Paginated};
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    ///
    /// The total number of items available
    ///
    #[serde(default)]
    pub total_items: usize,
    
    ///
    /// The the current page you are at
    ///
    #[serde(default)]
    pub current_page: usize,
    
    ///
    /// The the size of the batch
    ///
    #[serde(default)]
    pub page_size: usize,

    ///
    /// The total number of pages
    ///
    #[serde(default)]
    pub total_pages: usize,

    ///
    /// The url to get the next batch of items
    ///
    pub next_page: Option<String>,

    ///
    /// The url to get the previous batch of items
    ///
    pub prev_page: Option<String>,
}

pub async fn send_request<T: DeserializeOwned>(
    url: impl IntoUrl,
    params: impl IntoIterator<Item = (String, String)>,
) -> anyhow::Result<T> {
    let txt = Client::new()
        .get(url)
        .header("Content-Type", "application/json")
        .query(&params.into_iter().collect::<Vec<_>>())
        .send()
        .await?
        .text()
        .await?;

    fs::write(".DEBUG", &txt).expect("Valid debug write!");

    serde_json::from_str(&txt).map_err(anyhow::Error::from)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::api::types::Creator;
    use futures::StreamExt;

    use super::{paginated::Paginated, send_request, types::Model};

    async fn creators(query: impl Into<Option<String>>) -> anyhow::Result<Paginated<Creator>> {
        send_request(
            "https://civitai.com/api/v1/creators",
            query.into().into_iter().map(|q| ("query".to_string(), q)),
        )
        .await
    }

    #[tokio::test]
    async fn test_creator_fetch() -> anyhow::Result<()> {
        let c = creators("a".to_string()).await?;
        let mut c = c.into_stream();

        let mut i = 0;

        while let Some(user) = c.next().await {
            i += 1;
            println!("User({i}) => {user:?}");
        }

        Ok(())
    }
}
