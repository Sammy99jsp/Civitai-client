#![allow(non_camel_case_types)]
use std::collections::HashMap;

use crate::api::{types::{model::Image, Nsfw, Period}, paginated::Paginated};

use super::{Endpoint, MapLike};

#[derive(Debug, Default)]
pub enum Sort {
    MostReactions,
    MostComments,
    #[default]
    Newest,
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Sort::MostReactions => "MostReactions",
            Sort::MostComments => "MostComments",
            Sort::Newest => "Newest",
        }
        .to_string()
    }
}

#[derive(Debug, Default)]
pub struct Params {
    pub query: Option<String>,
    pub limit: Option<usize>,
    pub post_id: Option<usize>,
    pub model_id: Option<usize>,
    pub model_version_id: Option<usize>,
    pub username: Option<String>,
    pub nsfw: Option<Nsfw>,
    pub sort: Option<Sort>,
    pub period: Option<Period>,
}

impl MapLike for Params {
    fn into_map(self) -> std::collections::HashMap<String, String> {
        [
            ("query", self.query),
            ("limit", self.limit.map(|a| a.to_string())),
            ("postId", self.post_id.map(|a| a.to_string())),
            ("modelId", self.model_id.map(|a| a.to_string())),
            ("modelVersionId", self.model_version_id.map(|a| a.to_string())),
            ("username", self.username.map(|a| a.to_string())),
            ("sort", self.sort.map(|a| a.to_string())),
            ("period", self.period.map(|a| a.to_string())),
        ]
        .into_iter()
        .filter_map(|(k, v)| v.map(|v| (k.to_string(), v)))
        .collect()
    }
}

pub struct images;

impl Endpoint for images {
    const URL: &'static str = "https://civitai.com/api/v1/images";

    type Params = Params;

    type Response = Paginated<Image>;
}
