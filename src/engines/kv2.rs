use std::collections::HashMap;
use crate::client::VaultClient;
use crate::engines::ResponseMetadata;

const DEFAULT_PATH_KV2: &str = "secret";

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2Metadata {
    created_time: String,
    deletion_time: String,
    destroyed: bool,
    version: i32
}

type KvData = HashMap<String, String>;
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2Data {
    data: KvData,
    metadata: Kv2Metadata
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_versions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_version_after: Option<String>
}

#[derive(Serialize, Debug)]
pub struct KvOption {
    cas: Option<i32>
}

#[derive(Serialize, Debug, Default)]
pub struct PutKv2Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<KvOption>,
    pub data: KvData
}

#[derive(Serialize)]
pub struct UndeleteKv2VersionsRequest {
    pub versions: Vec<i32>
}

#[derive(Deserialize, Debug)]
pub struct PutKv2Response {
    pub data: Kv2Metadata
}

#[derive(Deserialize, Debug)]
pub struct GetKv2Response {
    #[serde(flatten)]
    meta: ResponseMetadata,
    pub data: Kv2Data,
}

#[derive(Deserialize, Debug)]
pub struct ReadKv2ConfigResponse {
    #[serde(flatten)]
    meta: ResponseMetadata,
    pub data: Kv2Config
}

impl VaultClient {
    fn data_url(&self, path: Option<&str>, kv: &str, version: Option<i32>) -> String {
        match version {
            Some(v) => {
                format!("{}/{}/data/{}?version={}", self.endpoint, path.unwrap_or(DEFAULT_PATH_KV2), kv, v)
            }
            None => {
                format!("{}/{}/data/{}", self.endpoint, path.unwrap_or(DEFAULT_PATH_KV2), kv)
            }
        }
    }

    pub async fn put_config(&self, mount: Option<&str>, data: &Kv2Config) -> reqwest::Result<reqwest::StatusCode> {
        Ok(self.http_client.post(
            &format!("{}/{}/config", self.endpoint, mount.unwrap_or(DEFAULT_PATH_KV2)))
            .json(data)
            .send().await?
            .status())
    }

    pub async fn get_config(&self, mount: Option<&str>) -> reqwest::Result<ReadKv2ConfigResponse> {
        self.http_client.get(
            &format!("{}/{}/config", self.endpoint, mount.unwrap_or(DEFAULT_PATH_KV2)))
            .send().await?
            .json::<ReadKv2ConfigResponse>().await
    }

    pub async fn put_kv(&self, mount: Option<&str>, kv: &str, data: &PutKv2Request) -> reqwest::Result<PutKv2Response> {
        self.http_client.post(self.data_url(mount, kv, None).as_str())
            .json(data)
            .send().await?
            .json::<PutKv2Response>().await
    }

    pub async fn get_kv(&self, mount: Option<&str>, kv: &str, version: Option<i32>) -> reqwest::Result<GetKv2Response> {
        self.http_client.get(self.data_url(mount, kv, version).as_str())
            .query(&[("version", version)])
            .send().await?
            .json::<GetKv2Response>().await
    }

    // pub async fn undelete_kv2_versions(&self, mount: Option<&str>, kv: &str, versions: Vec<i32>) -> impl std::future::Future {
    //     let url = format!("{}/{}/undelete/{}", self.endpoint, mount.unwrap_or(VAULT_DEFAULT_MOUNTPATH), kv);
    //     let req = UndeleteKv2VersionsRequest {
    //         versions: versions
    //     };
    //     self.post(&url, req)
    // }
}
