use league_client_connector::{LeagueClientConnector, RiotLockFile};
use native_tls::TlsConnector;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async_tls_with_config, Connector};
use ureq::{Agent, AgentBuilder};

use ugg_types::client_runepage::{NewRunePage, RunePage, RunePages};
use ugg_types::client_summoner::ClientSummoner;

use wamp_async::{Client, ClientConfig, GenericFuture};

#[derive(Error, Debug)]
pub enum LOLClientError {
    #[error("Unable to create TLS connector for League client")]
    TlsInitError(#[from] native_tls::Error),
    #[error("Unable to construct HTTP request")]
    HttpError(#[from] http::Error),
    #[error("Unable to read lockfile")]
    LockfileReadError(#[from] league_client_connector::LeagueConnectorError),
    #[error("Websocket could not connect")]
    WebsocketError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Linux is not supported")]
    LinuxNotSupported,
}

pub struct AsyncLOLClientAPI {
    agent: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>,
    lockfile: RiotLockFile,
}

impl AsyncLOLClientAPI {
    pub async fn new() -> Result<AsyncLOLClientAPI, LOLClientError> {
        if cfg!(target_os = "linux") {
            return Err(LOLClientError::LinuxNotSupported);
        }

        let lockfile = LeagueClientConnector::parse_lockfile()?;
        let connector = Connector::NativeTls(
            TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .build()?,
        );

        let request = http::Request::builder()
            .uri(format!("wss://127.0.0.1:{}", lockfile.port))
            .header("Authorization", format!("Basic {}", lockfile.b64_auth))
            .header("Host", format!("127.0.0.1:{}", lockfile.port))
            .header("Upgrade", "websocket")
            .header("Connection", "upgrade")
            .header("Sec-Websocket-Key", "lcu")
            .header("Sec-Websocket-Version", "13")
            .body(())?;
        let (agent, _) =
            connect_async_tls_with_config(request, None, false, Some(connector)).await?;

        Ok(AsyncLOLClientAPI { agent, lockfile })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::{SinkExt, StreamExt};

    #[tokio::test]
    async fn test_new() {
        let _ = env_logger::builder().is_test(true).try_init();

        let client = AsyncLOLClientAPI::new().await.unwrap();

        let (mut writer, mut reader) = client.agent.split();

        writer
            .send(tokio_tungstenite::tungstenite::Message::Text(
                "[5, \"OnJsonApiEvent_lol-champ-select_v1_session\"]".into(),
            ))
            .await
            .unwrap();

        while let Some(msg) = reader.next().await {
            dbg!(msg);
        }
    }
}
