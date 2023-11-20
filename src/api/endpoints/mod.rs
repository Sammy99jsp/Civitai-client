pub mod creators;
pub mod models;
pub mod images;

use std::collections::HashMap;

use futures::Future;
use serde::de::DeserializeOwned;

use super::send_request;

pub trait MapLike
where
    Self: Sized,
{
    fn into_map(self) -> HashMap<String, String>;
}

pub trait Endpoint: Sized {
    const URL: &'static str;
    type Params: MapLike + Default;
    type Response: DeserializeOwned;

    fn get(params: Self::Params) -> impl Future<Output = anyhow::Result<Self::Response>> {
        send_request::<Self::Response>(Self::URL, params.into_map())
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use crate::api::endpoints::Endpoint;

    use super::creators::creators;
    #[tokio::test]
    async fn test_creators() -> anyhow::Result<()> {
        let mut c = creators::get(Default::default()).await?.into_stream();

        while let Some(creator) = c.next().await {
            println!("{creator:?}")
        }

        Ok(())
    }

    use super::models::{models, Params as ModelParam};
    #[tokio::test]
    async fn test_models() -> anyhow::Result<()> {
        let mut c = models::get(ModelParam {
            query: "happy".to_string().into(),
            ..Default::default()
        })
        .await?
        .into_stream();

        while let Some(model) = c.next().await {
            println!("{model:#?}")
        }

        Ok(())
    }
}
