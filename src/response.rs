use bytes::Bytes;
use futures_util::TryFutureExt;
use serde::de::DeserializeOwned;
use serde::Deserialize;
pub struct VaultResponse(pub reqwest::Response);

use reqwest::StatusCode;

#[derive(Deserialize, Serialize, Default, Debug)]
struct VaultResponseMetadata {
    request_id: String,
    lease_id: String,
    renewable: bool,
    lease_duration: i32,
    wrap_info: Option<String>,
    warnings: Option<String>,
    auth: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct VaultData<T> {
    #[serde(flatten)]
    meta: VaultResponseMetadata,
    pub data: T
}

impl VaultResponse {

    pub async fn bytes(self) -> reqwest::Result<Bytes> {
        self.0.bytes().await
    }

    pub async fn parse<T: DeserializeOwned>(self) -> crate::error::Result<T> {
        let full = self.bytes().map_err(crate::error::reqwest).await?;
        serde_json::from_slice(&full).map_err(crate::error::decode)
    }

    pub fn status(&self) -> StatusCode {
        self.0.status()
    }
}
