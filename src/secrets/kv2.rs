use crate::client::VaultClient;
use crate::response::VaultSecret;
use std::collections::HashMap;

const DEFAULT_PATH_KV2: &str = "secret";

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct VaultKv2Metadata {
    created_time: String,
    deletion_time: String,
    destroyed: bool,
    version: Option<i32>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct VaultKv2SecretMetadataDetail {
    created_time: String,
    current_version: i32,
    max_versions: i32,
    oldest_version: i32,
    updated_time: String,
    versions: HashMap<String, String>,
}

type KvData = HashMap<String, String>;
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2Data {
    data: KvData,
    metadata: VaultKv2Metadata,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2KeyList {
    keys: Vec<String>,
    metadata: VaultKv2Metadata,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct VaultKv2Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_versions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_version_after: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct VaultKv2Options {
    cas: Option<i32>,
}

#[derive(Serialize, Debug, Default)]
pub struct VaultPutKv2Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<VaultKv2Options>,
    pub data: KvData,
}

#[derive(Serialize)]
pub struct UndeleteKv2VersionsRequest {
    pub versions: Vec<i32>,
}

impl VaultClient {
    pub async fn put_kv_config(
        &self,
        mount: Option<&str>,
        data: &VaultKv2Config,
    ) -> crate::Result<()> {
        self.post(
            &format!("{}/config", mount.unwrap_or(DEFAULT_PATH_KV2)),
            data,
        )
        .await
        .and_then(|_| Ok(()))
    }

    pub async fn get_kv_config(
        &self,
        mount: Option<&str>,
    ) -> crate::Result<VaultSecret<VaultKv2Config>> {
        self.get(&format!("{}/config", mount.unwrap_or(DEFAULT_PATH_KV2)))
            .await?
            .parse::<VaultSecret<VaultKv2Config>>()
            .await
    }

    pub async fn get_kv_version(
        &self,
        mount: Option<&str>,
        kv: &str,
        version: Option<i32>,
    ) -> crate::Result<VaultSecret<Kv2Data>> {
        match version {
            Some(v) => {
                self.get_with_query(
                    &format!("{}/data/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
                    &[("version", v)],
                )
                .await?
                .parse::<VaultSecret<Kv2Data>>()
                .await
            }
            None => {
                self.get(&format!(
                    "{}/data/{}",
                    mount.unwrap_or(DEFAULT_PATH_KV2),
                    kv
                ))
                .await?
                .parse::<VaultSecret<Kv2Data>>()
                .await
            }
        }
    }

    pub async fn put_kv(
        &self,
        mount: Option<&str>,
        kv: &str,
        data: &VaultPutKv2Request,
    ) -> crate::Result<VaultSecret<VaultKv2Metadata>> {
        self.post(
            &format!("{}/data/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
            data,
        )
        .await?
        .parse::<VaultSecret<VaultKv2Metadata>>()
        .await
    }

    pub async fn delete_kv_versions(
        &self,
        mount: Option<&str>,
        kv: &str,
        versions: Vec<i32>,
    ) -> crate::Result<VaultSecret<()>> {
        self.delete(
            &format!("{}/delete/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
            versions,
        )
        .await?
        .parse::<VaultSecret<()>>()
        .await
    }

    pub async fn undelete_kv_versions(
        &self,
        mount: Option<&str>,
        kv: &str,
        versions: Vec<i32>,
    ) -> crate::Result<VaultSecret<()>> {
        self.post(
            &format!("{}/undelete/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
            versions,
        )
        .await?
        .parse::<VaultSecret<()>>()
        .await
    }

    pub async fn destroy_kv_versions(
        &self,
        mount: Option<&str>,
        kv: &str,
        versions: Vec<i32>,
    ) -> crate::Result<VaultSecret<()>> {
        self.post(
            &format!("{}/destroy/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
            versions,
        )
        .await?
        .parse::<VaultSecret<()>>()
        .await
    }

    pub async fn list_kv(
        &self,
        mount: Option<&str>,
        path: &str,
    ) -> crate::Result<VaultSecret<Kv2KeyList>> {
        self.list(&format!(
            "{}/metadata/{}",
            mount.unwrap_or(DEFAULT_PATH_KV2),
            path
        ))
        .await?
        .parse::<VaultSecret<Kv2KeyList>>()
        .await
    }

    pub async fn get_kv_metadata(
        &self,
        mount: Option<&str>,
        path: &str,
    ) -> crate::Result<VaultSecret<VaultKv2SecretMetadataDetail>> {
        self.get(&format!("{}/metadata/{}", mount.unwrap_or(DEFAULT_PATH_KV2), path))
            .await?
            .parse::<VaultSecret<VaultKv2SecretMetadataDetail>>()
            .await
    }
}
