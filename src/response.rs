use futures_util::TryFutureExt;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
pub(crate) struct Response(pub(crate) reqwest::Response);

use bytes::Bytes;

#[derive(Deserialize, Serialize, Debug)]
pub struct SecretAuth {
    pub client_token: String,
    pub accessor: String,
    pub policies: Vec<String>,
    pub token_policies: Vec<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub orphan: bool,
    pub entity_id: String,
    pub lease_duration: i32,
    pub renewable: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SecretWrapInfo {
    pub token: String,
    pub accessor: String,
    pub ttl: i32,
    pub creation_time: String,
    pub creation_path: String,
    pub wrapped_accessor: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Secret<T> {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: i32,
    pub data: Option<T>,
    pub warnings: Option<Vec<String>>,
    pub auth: Option<SecretAuth>,
    pub wrap_infos: Option<SecretWrapInfo>,
}

impl Response {
    pub async fn bytes(self) -> reqwest::Result<Bytes> {
        self.0.bytes().await
    }

    pub async fn parse<T: DeserializeOwned>(self) -> crate::error::Result<T> {
        let full = self.bytes().map_err(crate::error::reqwest).await?;
        serde_json::from_slice(&full).map_err(crate::error::decode)
    }
}
