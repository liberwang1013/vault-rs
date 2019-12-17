use crate::client::VaultClient;
use crate::response::VaultData;
use std::collections::HashMap;
use reqwest::StatusCode;

const DEFAULT_PATH_KV2: &str = "secret";

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2Metadata {
    created_time: String,
    deletion_time: String,
    destroyed: bool,
    version: i32,
}

type KvData = HashMap<String, String>;
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2Data {
    data: KvData,
    metadata: Kv2Metadata,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_versions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_version_after: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct KvOption {
    cas: Option<i32>,
}

#[derive(Serialize, Debug, Default)]
pub struct PutKv2Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<KvOption>,
    pub data: KvData,
}

#[derive(Serialize)]
pub struct UndeleteKv2VersionsRequest {
    pub versions: Vec<i32>,
}

impl VaultClient {
    pub async fn put_config(
        &self,
        mount: Option<&str>,
        data: &Kv2Config,
    ) -> crate::error::Result<StatusCode> {
        self.post(
            &format!("{}/config", mount.unwrap_or(DEFAULT_PATH_KV2)),
            data,
        )
        .await
        .and_then(|rsp| Ok(rsp.status()))
    }

    pub async fn get_config(
        &self,
        mount: Option<&str>,
    ) -> crate::error::Result<VaultData<Kv2Config>> {
        self.get(&format!("{}/config", mount.unwrap_or(DEFAULT_PATH_KV2)))
            .await?
            .parse::<VaultData<Kv2Config>>()
            .await
    }

    pub async fn put_kv(
        &self,
        mount: Option<&str>,
        kv: &str,
        data: &PutKv2Request,
    ) -> crate::error::Result<VaultData<Kv2Metadata>> {
        self.post(
            &format!("{}/data/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
            data,
        )
        .await?
        .parse::<VaultData<Kv2Metadata>>()
        .await
    }

    pub async fn get_kv(
        &self,
        mount: Option<&str>,
        kv: &str,
    ) -> crate::error::Result<VaultData<Kv2Data>> {
        self.get(&format!(
            "{}/data/{}",
            mount.unwrap_or(DEFAULT_PATH_KV2),
            kv
        ))
        .await?
        .parse::<VaultData<Kv2Data>>()
        .await
    }

    pub async fn get_kv_with_version(
        &self,
        mount: Option<&str>,
        kv: &str,
        version: i32,
    ) -> crate::error::Result<VaultData<Kv2Data>> {
        self.get_with_query(
            &format!("{}/data/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
            &[("version", version)],
        )
        .await?
        .parse::<VaultData<Kv2Data>>()
        .await
    }

    // pub async fn undelete_kv2_versions(&self, mount: Option<&str>, kv: &str, versions: Vec<i32>) -> impl std::future::Future {
    //     let url = format!("{}/{}/undelete/{}", self.endpoint, mount.unwrap_or(VAULT_DEFAULT_MOUNTPATH), kv);
    //     let req = UndeleteKv2VersionsRequest {
    //         versions: versions
    //     };
    //     self.post(&url, req)
    // }
}
