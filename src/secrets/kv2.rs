use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub cas_required: Option<bool>,
    pub delete_version_after: Option<i32>,
    pub max_versions: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigResponse {
    pub cas_required: Option<bool>,
    pub delete_version_after: Option<i32>,
    pub max_versions: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct DataRequest {
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
    pub options: Option<serde_json::Map<String, serde_json::Value>>,
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct DataResponse {
    pub created_time: Option<String>,
    pub custom_metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub deletion_time: Option<String>,
    pub destroyed: Option<bool>,
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct DeleteRequest {
    pub versions: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct DestroyRequest {
    pub versions: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct MetadataRequest {
    pub cas_required: Option<bool>,
    pub custom_metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub delete_version_after: Option<i32>,
    pub max_versions: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct MetadataResponse {
    pub cas_required: Option<bool>,
    pub created_time: Option<String>,
    pub current_version: Option<i64>,
    pub custom_metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub delete_version_after: Option<i32>,
    pub max_versions: Option<i64>,
    pub oldest_version: Option<i64>,
    pub updated_time: Option<String>,
    pub versions: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct SubkeysResponse {
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub subkeys: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct UndeleteRequest {
    pub versions: Option<Vec<i32>>,
}