use std::collections::HashMap;
use crate::client::VaultClient;
//use reqwest::Response;

const VAULT_DEFAULT_MOUNTPATH: &str = "secret";

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Metadata {
    created_time: String,
    deletion_time: String,
    destroyed: bool,
    version: i32
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2Data {
    data: HashMap<String, String>,
    metadata: Metadata
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ConfigData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_versions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_version_after: Option<String>
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct SecretMeta {
    request_id: String,
    lease_id: String,
    renewable: bool,
    lease_duration: i32,
    wrap_info: Option<String>,
    warnings: Option<String>,
    auth: Option<String>
}

#[derive(Serialize, Debug)]
pub struct OptionSetting {
    cas: Option<i32>
}

#[derive(Serialize, Debug, Default)]
pub struct PutKV2Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OptionSetting>,

    pub data: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
pub struct PutKV2Response {
    pub data: Metadata
}

#[derive(Deserialize, Debug)]
pub struct GetKV2Response {
    #[serde(flatten)]
    meta: SecretMeta,
    pub data: Kv2Data,
}

#[derive(Deserialize, Debug)]
pub struct ReadKV2ConfigResponse {
    #[serde(flatten)]
    meta: SecretMeta,
    pub data: ConfigData
}

impl VaultClient {
    fn data_url(&self, path: Option<&str>, kv: &str, version: Option<i32>) -> String {
        match version {
            Some(v) => {
                format!("{}/v1/{}/data/{}?version={}", self.endpoint, path.unwrap_or(VAULT_DEFAULT_MOUNTPATH), kv, v)
            }
            None => {
                format!("{}/v1/{}/data/{}", self.endpoint, path.unwrap_or(VAULT_DEFAULT_MOUNTPATH), kv)
            }
        }
    }

    pub async fn put_config(&self, mount: Option<&str>, data: &ConfigData) -> reqwest::Result<reqwest::StatusCode> {
        Ok(self.http_client.post(
            &format!("{}/v1/{}/config", self.endpoint, mount.unwrap_or(VAULT_DEFAULT_MOUNTPATH)))
            .json(data)
            .send().await?
            .status())
    }

    pub async fn get_config(&self, mount: Option<&str>) -> reqwest::Result<ReadKV2ConfigResponse> {
        self.http_client.get(
            &format!("{}/v1/{}/config", self.endpoint, mount.unwrap_or(VAULT_DEFAULT_MOUNTPATH)))
            .send().await?
            .json::<ReadKV2ConfigResponse>().await
    }

    pub async fn put_kv(&self, mount: Option<&str>, kv: &str, data: &PutKV2Request) -> reqwest::Result<PutKV2Response> {
        self.http_client.post(self.data_url(mount, kv, None).as_str())
            .json(data)
            .send().await?
            .json::<PutKV2Response>().await
    }

    pub async fn get_kv(&self, mount: Option<&str>, kv: &str, version: Option<i32>) -> reqwest::Result<GetKV2Response> {
        self.http_client.get(self.data_url(mount, kv, version).as_str())
            .query(&[("version", version)])
            .send().await?
            .json::<GetKV2Response>().await
    }

    // pub fn delete_latest_kv(&self, path: Option<&str>, kv: &str) -> Result<()> {
    //     let rsp = self.http_client.delete(self.data_url(path, kv)?)
    //         .send()?
    //         .error_for_status()?;
    //     Ok(())
    // }

}
