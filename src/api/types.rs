use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    ///
    /// The username of the creator
    ///
    pub username: String,

    ///
    /// The amount of models linked to this user
    ///
    pub model_count: Option<usize>,

    ///
    /// Url to get all models from this user
    ///
    pub link: String,
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
pub enum Nsfw {
    #[default]
    #[serde(rename = "None")]
    None,

    #[serde(rename = "Soft")]
    Soft,

    #[serde(rename = "Mature")]
    Mature,

    #[serde(rename = "x")]
    X,
}

pub mod model {
    use std::default;

    use chrono::{DateTime, Utc};
    use serde::Deserialize;

    use super::{super::utils::datetime, Nsfw};

    #[derive(Debug, Deserialize, Clone, Copy)]
    pub enum Type {
        #[serde(rename = "Checkpoint")]
        Checkpoint,

        #[serde(rename = "TextualInversion")]
        TextualInversion,

        #[serde(rename = "Hypernetwork")]
        Hypernetwork,

        #[serde(rename = "AestheticGradient")]
        AestheticGradient,

        #[serde(rename = "LORA")]
        Lora,

        #[serde(rename = "Controlnet")]
        Controlnet,

        #[serde(rename = "Poses")]
        Poses,
    }

    #[derive(Debug, Deserialize, Clone, Copy)]
    pub enum Mode {
        #[serde(rename = "Archived")]
        Archived,
        #[serde(rename = "TakenDown")]
        TakenDown,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Creator {
        ///
        /// The name of the creator
        ///
        pub username: String,

        ///
        /// The url of the creators avatar
        ///
        pub image: Option<String>,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Stats {
        ///
        /// The number of downloads the model has
        ///
        pub download_count: usize,

        ///
        /// The number of favorites the model has
        ///
        pub favorite_count: usize,

        ///
        /// The number of comments the model has
        ///
        pub comment_count: usize,

        ///
        /// The number of ratings the model has
        ///
        pub rating_count: usize,

        ///
        /// The average rating of the model
        ///
        pub rating: f64,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Version {
        ///
        /// The identifier for the model version
        ///
        pub id: usize,

        ///
        /// The name of the model version
        ///
        pub name: String,

        ///
        /// The description of the model version (usually a changelog)
        ///
        pub description: Option<String>,

        ///
        /// The date in which the version was created
        ///
        #[serde(deserialize_with = "datetime::deserialize_option")]
        pub created_at: Option<DateTime<Utc>>,

        ///
        /// The download url to get the model file for this specific version
        ///
        pub download_url: String,

        ///
        /// The words used to trigger the model
        ///
        pub trained_words: Vec<String>,

        ///
        /// Model files for this version
        ///
        pub files: Vec<File>,
    }

    #[derive(Debug, Deserialize, Clone, Copy)]
    pub enum ScanResult {
        #[serde(rename = "Pending")]
        Pending,

        #[serde(rename = "Success")]
        Success,

        #[serde(rename = "Danger")]
        Danger,

        #[serde(rename = "Error")]
        Error,
    }

    #[derive(Debug, Deserialize, Clone, Copy)]
    pub enum FloatingPoint {
        #[serde(rename = "fp16")]
        Fp16,

        #[serde(rename = "fp32")]
        Fp32,
    }

    #[derive(Debug, Deserialize, Clone, Copy)]
    pub enum Size {
        #[serde(rename = "full")]
        Full,

        #[serde(rename = "pruned")]
        Pruned,
    }

    #[derive(Debug, Deserialize, Clone, Default)]
    pub enum Format {
        #[serde(rename = "SafeTensor")]
        SafeTensor,

        #[serde(rename = "PickleTensor")]
        PickleTensor,

        #[serde(rename = "Other")]
        #[default]
        Other,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct FileMetadata {
        ///
        /// The specified floating point for the file
        ///
        pub fp: FloatingPoint,

        ///
        /// The specified model size for the file
        ///
        pub size: Option<Size>,

        ///
        /// The specified model format for the file
        ///
        pub format: Option<Format>,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Hashes {
        #[serde(rename = "AutoV1")]
        auto_v1: Option<String>,

        #[serde(rename = "AutoV2")]
        auto_v2: Option<String>,

        #[serde(rename = "SHA256")]
        sha256: Option<String>,

        #[serde(rename = "CRC32")]
        crc32: Option<String>,

        #[serde(rename = "BLAKE3")]
        blake3: Option<String>,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct File {
        ///
        /// The identifier for the model file
        ///
        pub id: usize,

        ///
        /// The size of the model file
        ///
        #[serde(rename = "sizeKb")]
        pub size_kb: Option<f64>,

        ///
        /// Name of the model file
        ///
        pub name: String,

        ///
        /// Status of the pickle scan
        ///
        pub pickle_scan_result: Option<ScanResult>,

        ///
        /// Message from pickle scan
        ///
        pub pickle_scan_message: Option<String>,

        ///
        /// Status of the virus scan
        ///
        pub virus_scan_result: Option<ScanResult>,

        ///
        /// Message from virus scan
        ///
        pub virus_scan_message: Option<String>,

        ///
        /// The date in which the file was scanned
        ///
        #[serde(deserialize_with = "datetime::deserialize_option")]
        pub scanned_at: Option<DateTime<Utc>>,

        ///
        /// Hashes of this file in various formats
        ///
        pub hashes: Hashes,

        ///
        /// If the file is the primary file for the model version
        ///
        pub primary: Option<bool>,

        ///
        /// Direct link to this file's download.
        ///
        pub download_url: String,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Image {
        ///
        /// The url for the image
        ///
        pub url: String,

        ///
        /// Whether or not the image is NSFW
        ///
        #[serde(default)]
        pub nsfw: Nsfw,

        ///
        /// Width of the image in pixels
        ///
        pub width: usize,

        ///
        /// Height of the image in pixels
        ///
        pub height: usize,
        // TODO: Add image metadata HERE!
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    ///
    /// The identifier for the model
    ///
    pub id: usize,

    ///
    /// The name of the model
    ///
    pub name: String,

    ///
    /// The description of the model (HTML)
    ///
    pub description: String,

    ///
    /// The model type
    ///
    #[serde(rename = "type")]
    pub _type: model::Type,

    ///
    /// The tags associated with the model
    ///
    pub tags: Vec<String>,

    ///
    /// The mode in which the model is currently on
    ///
    pub mode: Option<model::Mode>,

    ///
    /// Creator of this model
    ///
    pub creator: model::Creator,

    ///
    /// Stats of this model
    ///
    pub stats: model::Stats,

    ///
    /// Versions of this model
    ///
    #[serde(rename = "modelVersions")]
    pub versions: Vec<model::Version>,
}
