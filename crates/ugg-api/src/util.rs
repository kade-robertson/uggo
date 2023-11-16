use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub fn sha256(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))
}

#[derive(Error, Debug)]
pub enum ReadCacheError {
    #[error("Could not open file")]
    FileError(#[from] std::io::Error),
    #[error("Could not parse file")]
    ParseError(#[from] simd_json::Error),
}

pub fn read_from_cache<T: DeserializeOwned>(
    cache_dir: &PathBuf,
    filename: &str,
) -> Result<T, ReadCacheError> {
    let file_path = Path::new(cache_dir).join(format!("{}.json", sha256(filename)));
    let file = File::open(file_path)?;

    simd_json::serde::from_reader::<File, T>(file).map_err(|e| e.into())
}

#[derive(Error, Debug)]
pub enum WriteCacheError {
    #[error("Could not open file")]
    FileError(#[from] std::io::Error),
    #[error("Could not serialize data or write to file")]
    ParseError(#[from] simd_json::Error),
}

pub fn write_to_cache<T: Serialize>(
    cache_dir: &PathBuf,
    filename: &str,
    data: &T,
) -> Result<(), WriteCacheError> {
    let file_path = Path::new(cache_dir).join(format!("{}.json", sha256(filename)));
    let file = File::create(file_path)?;

    Ok(simd_json::serde::to_writer(file, data)?)
}

pub fn clear_cache(cache_dir: &PathBuf, filename: &str) {
    let file_path = Path::new(cache_dir).join(format!("{}.json", sha256(filename)));
    if file_path.exists() {
        fs::remove_file(file_path).ok();
    }
}
