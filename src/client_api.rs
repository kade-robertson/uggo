use lazy_static::lazy_static;
use league_client_connector::RiotLockFile;
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;

use crate::types::client_summoner::ClientSummoner;

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
}

fn get_data<T: DeserializeOwned>(url: &String, auth: &String) -> Option<T> {
    match CLIENT
        .get(url)
        .header(AUTHORIZATION, format!("Basic {}", auth))
        .send()
    {
        Ok(response) => {
            if response.status().is_success() {
                let json_data = response.json::<T>();
                match json_data {
                    Ok(json) => Some(json),
                    Err(_) => None,
                }
            } else {
                return None;
            }
        }
        Err(_) => None,
    }
}

pub fn get_summoner_info(lockfile: &RiotLockFile) -> Option<Box<ClientSummoner>> {
    match get_data::<ClientSummoner>(
        &format!(
            "https://127.0.0.1:{}/lol-summoner/v1/current-summoner",
            lockfile.port
        ),
        &lockfile.b64_auth,
    ) {
        Some(data) => Some(Box::new(data)),
        None => None,
    }
}
