use crate::client::Client;
use crate::response::Secret;
use std::collections::HashMap;

const DEFAULT_PATH_KV2: &str = "secret";

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct KV2Config {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cas_required: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_versions: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub delete_version_after: Option<String>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct KV2VersionMetadata {
  created_time: String,
  deletion_time: String,
  destroyed: bool,
  version: Option<i32>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct KV2Metadata {
  created_time: String,
  current_version: i32,
  max_versions: i32,
  oldest_version: i32,
  updated_time: String,
  versions: HashMap<String, KV2VersionMetadata>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct KV2VersionData {
  data: HashMap<String, String>,
  metadata: KV2VersionMetadata,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Kv2KeyList {
  keys: Vec<String>,
  metadata: KV2VersionMetadata,
}

#[derive(Serialize, Debug)]
pub struct KV2Options {
  cas: Option<i32>,
}

#[derive(Serialize, Debug, Default)]
pub struct PutKV2Request {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<KV2Options>,
  pub data: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct UndeleteKV2VersionsRequest {
  pub versions: Vec<i32>,
}

impl Client {
  //
  pub async fn put_kv_config(
    &self,
    mount: Option<&str>,
    data: &KV2Config,
  ) -> crate::Result<()> {
    self
      .post(
        &format!("{}/config", mount.unwrap_or(DEFAULT_PATH_KV2)),
        data,
      )
      .await
      .and_then(|_| Ok(()))
  }

  pub async fn get_kv_config(
    &self,
    mount: Option<&str>,
  ) -> crate::Result<Secret<KV2Config>> {
    self
      .get(&format!("{}/config", mount.unwrap_or(DEFAULT_PATH_KV2)))
      .await?
      .parse::<Secret<KV2Config>>()
      .await
  }

  pub async fn get_kv_version(
    &self,
    mount: Option<&str>,
    kv: &str,
    version: Option<i32>,
  ) -> crate::Result<Secret<KV2VersionData>> {
    match version {
      Some(v) => {
        self
          .get_with_query(
            &format!("{}/data/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
            &[("version", v)],
          )
          .await?
          .parse::<Secret<KV2VersionData>>()
          .await
      }
      None => {
        self
          .get(&format!(
            "{}/data/{}",
            mount.unwrap_or(DEFAULT_PATH_KV2),
            kv
          ))
          .await?
          .parse::<Secret<KV2VersionData>>()
          .await
      }
    }
  }

  pub async fn put_kv(
    &self,
    mount: Option<&str>,
    kv: &str,
    data: &PutKV2Request,
  ) -> crate::Result<Secret<KV2VersionMetadata>> {
    self
      .post(
        &format!("{}/data/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
        data,
      )
      .await?
      .parse::<Secret<KV2VersionMetadata>>()
      .await
  }

  pub async fn delete_kv(
    &self,
    mount: Option<&str>,
    kv: &str,
  ) -> crate::Result<Secret<KV2VersionMetadata>> {
    self
      .delete(&format!(
        "{}/data/{}",
        mount.unwrap_or(DEFAULT_PATH_KV2),
        kv
      ))
      .await?
      .parse::<Secret<KV2VersionMetadata>>()
      .await
  }

  pub async fn delete_kv_versions(
    &self,
    mount: Option<&str>,
    kv: &str,
    versions: Vec<i32>,
  ) -> crate::Result<Secret<()>> {
    self
      .post(
        &format!("{}/delete/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
        versions,
      )
      .await?
      .parse::<Secret<()>>()
      .await
  }

  pub async fn undelete_kv_versions(
    &self,
    mount: Option<&str>,
    kv: &str,
    versions: Vec<i32>,
  ) -> crate::Result<Secret<()>> {
    self
      .post(
        &format!("{}/undelete/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
        versions,
      )
      .await?
      .parse::<Secret<()>>()
      .await
  }

  pub async fn destroy_kv_versions(
    &self,
    mount: Option<&str>,
    kv: &str,
    versions: Vec<i32>,
  ) -> crate::Result<Secret<()>> {
    self
      .post(
        &format!("{}/destroy/{}", mount.unwrap_or(DEFAULT_PATH_KV2), kv),
        versions,
      )
      .await?
      .parse::<Secret<()>>()
      .await
  }

  pub async fn list_kv(
    &self,
    mount: Option<&str>,
    path: &str,
  ) -> crate::Result<Secret<Kv2KeyList>> {
    self
      .list(&format!(
        "{}/metadata/{}",
        mount.unwrap_or(DEFAULT_PATH_KV2),
        path
      ))
      .await?
      .parse::<Secret<Kv2KeyList>>()
      .await
  }

  pub async fn get_kv_metadata(
    &self,
    mount: Option<&str>,
    path: &str,
  ) -> crate::Result<Secret<KV2Metadata>> {
    self
      .get(&format!(
        "{}/metadata/{}",
        mount.unwrap_or(DEFAULT_PATH_KV2),
        path
      ))
      .await?
      .parse::<Secret<KV2Metadata>>()
      .await
  }

  pub async fn put_kv_metadata(
    &self,
    mount: Option<&str>,
    path: &str,
    data: &KV2Config,
  ) -> crate::Result<()> {
    self
      .post(
        &format!("{}/metadata/{}", mount.unwrap_or(DEFAULT_PATH_KV2), path),
        data,
      )
      .await
      .and_then(|_| Ok(()))
  }

  pub async fn desstroy_kv(&self, mount: Option<&str>, path: &str) -> crate::Result<()> {
    self
      .delete(&format!(
        "{}/metedata/{}",
        mount.unwrap_or(DEFAULT_PATH_KV2),
        path
      ))
      .await
      .and_then(|_| Ok(()))
  }
}
