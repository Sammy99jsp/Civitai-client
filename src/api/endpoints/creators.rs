#![allow(non_camel_case_types)]
use crate::api::{paginated::Paginated, types::Creator};

use super::Endpoint;

#[derive(Debug, Default)]
pub struct Params {
    pub query: Option<String>
}

use super::MapLike;


impl MapLike for Params {
    fn into_map(self) -> std::collections::HashMap<String, String> {
        [
            ("query".to_string(), self.query)
        ].into_iter()
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .collect()
    }
}

pub struct creators;

impl Endpoint for creators {
    const URL: &'static str = "https://civitai.com/api/v1/creators";

    type Params = Params;
    type Response = Paginated<Creator>;
} 