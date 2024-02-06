use league_client_connector::{LeagueClientConnector, RiotLockFile};
use native_tls::TlsConnector;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;
use ureq::{Agent, AgentBuilder};

mod champ_select_session;
mod client_summoner;
mod lobby;

pub use champ_select_session::ChampSelectSession;
pub use client_summoner::ClientSummoner;
pub use lobby::{Lobby, QueueID};

use ugg_types::client_runepage::{NewRunePage, RunePage, RunePages};

#[derive(Error, Debug)]
pub enum LOLClientError {
    #[error("Unable to create TLS connector for League client")]
    TlsInitError(#[from] native_tls::Error),
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
            agent: AgentBuilder::new()
                .tls_connector(Arc::new(
                    TlsConnector::builder()
                        .danger_accept_invalid_certs(true)
                        .build()?,
                ))
                .build(),
            lockfile: LeagueClientConnector::parse_lockfile()?,
        })
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

    pub fn get_summoner_info(&self) -> Option<ClientSummoner> {
        self.get_data::<ClientSummoner>(&format!(
            "https://127.0.0.1:{}/lol-summoner/v1/current-summoner",
            self.lockfile.port
        ))
    }

    pub fn get_current_champion_id(&self, summoner_id: i64) -> Option<i64> {
        self.get_data::<ChampSelectSession>(&format!(
            "https://127.0.0.1:{}/lol-champ-select/v1/session",
            self.lockfile.port
        ))
        .and_then(|css| {
            css.my_team
                .into_iter()
                .find(|p| p.summoner_id == summoner_id)
                .map(|p| p.champion_id)
        })
    }

    pub fn get_current_queue_id(&self) -> Option<QueueID> {
        self.get_data::<Lobby>(&format!(
            "https://127.0.0.1:{}/lol-game-queues/v1/queues",
            self.lockfile.port
        ))
        .map(|l| l.game_config.queue_id)
    }

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
