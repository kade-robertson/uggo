use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};

pub fn sha256(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))
}

pub fn read_from_cache<T: DeserializeOwned>(cache_dir: &str, filename: &str) -> Option<T> {
    let file_path = Path::new(cache_dir).join(format!("{}.json", sha256(filename)));
    if file_path.exists() {
        match simd_json::serde::from_owned_value::<T>(simd_json::OwnedValue::String(
            fs::read_to_string(file_path).unwrap_or_default(),
        )) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    } else {
        None
    }
}

pub fn write_to_cache<T: Serialize>(cache_dir: &str, filename: &str, data: &T) {
    let file_path = Path::new(cache_dir).join(format!("{}.json", sha256(filename)));
    if let Ok(data) = simd_json::serde::to_string::<T>(data) {
        fs::write(file_path, data).ok();
    }
}

pub fn clear_cache(cache_dir: &str, filename: &str) {
    let file_path = Path::new(cache_dir).join(format!("{}.json", sha256(filename)));
    if file_path.exists() {
        fs::remove_file(file_path).ok();
    }
}
