#![allow(non_camel_case_types)]
use crate::api::{
    paginated::Paginated,
    types::{model::Type, Model, Nsfw, Period},
};

use super::Endpoint;

use crate::api::types::model;

use super::MapLike;

#[derive(Debug, Clone, Copy, Default)]
pub enum Sorting {
    #[default]
    HighestRated,
    MostDownloaded,
    Newest,
}

impl Sorting {
    pub fn value(self) -> &'static str {
        match self {
            Sorting::HighestRated => "Highest Rated",
            Sorting::MostDownloaded => "Most Downloaded",
            Sorting::Newest => "Newest",
        }
    }
}


#[derive(Debug, Default)]
pub struct Params {
    pub query: Option<String>,
    pub tag: Option<String>,
    pub username: Option<String>,
    pub types: Option<model::Type>,
    pub sort: Option<Sorting>,
    pub period: Option<Period>,
    pub rating: Option<f64>,
    pub nsfw: Option<Nsfw>,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Checkpoint => "Checkpoint",
            Type::TextualInversion => "TextualInversion",
            Type::Hypernetwork => "Hypernetwork",
            Type::AestheticGradient => "AestheticGradient",
            Type::Lora => "Lora",
            Type::Controlnet => "Controlnet",
            Type::Poses => "Poses",
        }.to_string()
    }
}

impl MapLike for Params {
    fn into_map(self) -> std::collections::HashMap<String, String> {
        [
            ("query".to_string(), self.query),
            ("tag".to_string(), self.tag),
            ("username".to_string(), self.username),
            (
                "types".to_string(),
                self.types.map(|a| a.to_string()),
            ),
            (
                "period".to_string(),
                self.period.map(|a| a.to_string()),
            ),
            ("rating".to_string(), self.rating.map(|a| a.to_string())),
            ("nsfw".to_string(), self.nsfw.map(|a| a.to_string())),
        ]
        .into_iter()
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .collect()
    }
}

pub struct models;

impl Endpoint for models {
    const URL: &'static str = "https://civitai.com/api/v1/models";

    type Params = Params;
    type Response = Paginated<Model>;
}
