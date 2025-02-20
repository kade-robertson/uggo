use league_client_connector::{LeagueClientConnector, RiotLockFile};
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use ureq::tls::TlsConfig;
use ureq::Agent;

use ugg_types::client_runepage::{NewRunePage, RunePage, RunePages};
use ugg_types::client_summoner::ClientSummoner;

#[derive(Error, Debug)]
pub enum LOLClientError {
    #[error("Unable to read lockfile")]
    LockfileReadError(#[from] league_client_connector::LeagueConnectorError),
    #[error("Linux is not supported")]
    LinuxNotSupported,
}

pub struct LOLClientAPI {
    agent: Agent,
    lockfile: RiotLockFile,
}

impl LOLClientAPI {
    pub fn new() -> Result<LOLClientAPI, LOLClientError> {
        if cfg!(target_os = "linux") {
            return Err(LOLClientError::LinuxNotSupported);
        }
        Ok(LOLClientAPI {
            agent: Agent::config_builder()
                .tls_config(
                    TlsConfig::builder()
                        .provider(ureq::tls::TlsProvider::NativeTls)
                        .disable_verification(true)
                        .build(),
                )
                .build()
                .into(),
            lockfile: LeagueClientConnector::parse_lockfile()?,
        })
    }

    fn get_data<T: DeserializeOwned>(&self, url: &str) -> Option<T> {
        match self
            .agent
            .get(url)
            .header(
                "Authorization",
                &format!("Basic {}", self.lockfile.b64_auth),
            )
            .call()
        {
            Ok(response) => {
                if response.status() == 200 {
                    response.into_body().read_json::<T>().ok()
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
            .header(
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
            .header(
                "Authorization",
                &format!("Basic {}", self.lockfile.b64_auth),
            )
            .send_json(data)
        {
            Err(_) | Ok(_) => (),
        }
    }

    #[must_use]
    pub fn get_summoner_info(&self) -> Option<ClientSummoner> {
        self.get_data::<ClientSummoner>(&format!(
            "https://127.0.0.1:{}/lol-summoner/v1/current-summoner",
            self.lockfile.port
        ))
    }

    #[must_use]
    pub fn get_current_rune_page(&self) -> Option<RunePage> {
        match self.get_data::<RunePages>(&format!(
            "https://127.0.0.1:{}/lol-perks/v1/pages",
            self.lockfile.port
        )) {
            Some(data) => {
                for page in &data {
                    if page.name.starts_with("uggo:") && page.is_deletable {
                        return Some(page.clone());
                    }
                }
                for page in &data {
                    if page.current && page.is_deletable {
                        return Some(page.clone());
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
