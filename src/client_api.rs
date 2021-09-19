use lazy_static::lazy_static;
use league_client_connector::RiotLockFile;
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::types::client_runepage::{NewRunePage, RunePage, RunePages};
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

fn delete_data(url: &String, auth: &String) {
    match CLIENT
        .delete(url)
        .header(AUTHORIZATION, format!("Basic {}", auth))
        .send()
    {
        Ok(_) => (),
        Err(_) => (),
    }
}

fn post_data<T: Serialize>(url: &String, auth: &String, data: &T) {
    match CLIENT
        .post(url)
        .header(AUTHORIZATION, format!("Basic {}", auth))
        .json(data)
        .send()
    {
        Ok(_) => (),
        Err(_) => (),
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

pub fn get_current_rune_page(lockfile: &RiotLockFile) -> Option<Box<RunePage>> {
    match get_data::<RunePages>(
        &format!("https://127.0.0.1:{}/lol-perks/v1/pages", lockfile.port),
        &lockfile.b64_auth,
    ) {
        Some(data) => {
            for page in data {
                if page.current && page.is_editable {
                    return Some(Box::new(page));
                }
            }
            return None;
        }
        None => None,
    }
}

pub fn update_rune_page(lockfile: &RiotLockFile, old_page_id: &i64, rune_page: &NewRunePage) {
    delete_data(
        &format!(
            "https://127.0.0.1:{}/lol-perks/v1/pages/{}",
            lockfile.port, old_page_id
        ),
        &lockfile.b64_auth,
    );
    post_data::<NewRunePage>(
        &format!("https://127.0.0.1:{}/lol-perks/v1/pages", lockfile.port),
        &lockfile.b64_auth,
        rune_page,
    );
}
