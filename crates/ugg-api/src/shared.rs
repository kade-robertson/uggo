use std::collections::HashMap;

use thiserror::Error;

pub type UggAPIVersions = HashMap<String, HashMap<String, String>>;

#[derive(Error, Debug)]
pub enum UggError {
    #[error("DDragon error")]
    DDragonError(#[from] ddragon::ClientError),
    #[error("HTTP request failed")]
    RequestError(#[from] Box<ureq::Error>),
    #[cfg(feature = "async")]
    #[error("HTTP request failed")]
    AsyncRequestError(#[from] Box<reqwest::Error>),
    #[error("JSON parsing failed")]
    ParseError(#[from] simd_json::Error),
    #[error("Missing region or rank entry")]
    MissingRegionOrRank,
    #[error("Missing role entry")]
    MissingRole,
    #[error("Unknown error occurred")]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct SupportedVersion {
    pub ddragon: String,
    pub ugg: String,
}
