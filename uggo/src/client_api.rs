use league_client_connector::RiotLockFile;
use native_tls::TlsConnector;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use ureq::{Agent, AgentBuilder};

use ugg_types::client_runepage::{NewRunePage, RunePage, RunePages};
use ugg_types::client_summoner::ClientSummoner;

pub struct ClientAPI {
    agent: Agent,
    lockfile: RiotLockFile,
}

impl ClientAPI {
    pub fn new(lockfile: RiotLockFile) -> ClientAPI {
        ClientAPI {
            agent: AgentBuilder::new()
                .tls_connector(Arc::new(
                    TlsConnector::builder()
                        .danger_accept_invalid_certs(true)
                        .build()
                        .unwrap(),
                ))
                .build(),
            lockfile,
        }
    }

    fn get_data<T: DeserializeOwned>(&self, url: &str) -> Option<T> {
        match self
            .agent
            .get(url)
            .set(
                "Authorization",
                &format!("Basic {}", self.lockfile.b64_auth),
            )
            .call()
        {
            Ok(response) => {
                if response.status() == 200 {
                    let json_data = response.into_json::<T>();
                    match json_data {
                        Ok(json) => Some(json),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn delete_data(&self, url: &str) {
        match self
            .agent
            .delete(url)
            .set(
                "Authorization",
                &format!("Basic {}", self.lockfile.b64_auth),
            )
            .call()
        {
            Err(_) | Ok(_) => (),
        }
    }

    fn post_data<T: Serialize>(&self, url: &str, data: &T) {
        match self
            .agent
            .post(url)
            .set(
                "Authorization",
                &format!("Basic {}", self.lockfile.b64_auth),
            )
            .send_json(data)
        {
            Err(_) | Ok(_) => (),
        }
    }

    pub fn get_summoner_info(&self) -> Option<Box<ClientSummoner>> {
        self.get_data::<ClientSummoner>(&format!(
            "https://127.0.0.1:{}/lol-summoner/v1/current-summoner",
            self.lockfile.port
        ))
        .map(Box::new)
    }

    pub fn get_current_rune_page(&self) -> Option<Box<RunePage>> {
        match self.get_data::<RunePages>(&format!(
            "https://127.0.0.1:{}/lol-perks/v1/pages",
            self.lockfile.port
        )) {
            Some(data) => {
                for page in &data {
                    if page.name.starts_with("uggo:") && page.is_deletable {
                        return Some(Box::new(page.clone()));
                    }
                }
                for page in &data {
                    if page.current && page.is_deletable {
                        return Some(Box::new(page.clone()));
                    }
                }
                None
            }
            None => None,
        }
    }

    pub fn update_rune_page(&self, old_page_id: i64, rune_page: &NewRunePage) {
        self.delete_data(&format!(
            "https://127.0.0.1:{}/lol-perks/v1/pages/{}",
            self.lockfile.port, old_page_id
        ));
        self.post_data::<NewRunePage>(
            &format!(
                "https://127.0.0.1:{}/lol-perks/v1/pages",
                self.lockfile.port
            ),
            rune_page,
        );
    }
}
