use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub credentials: Option<String>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct DecryptRequest {
    pub additional_authenticated_data: Option<String>,
    pub ciphertext: Option<String>,
    pub key_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct EncryptRequest {
    pub additional_authenticated_data: Option<String>,
    pub key_version: Option<i32>,
    pub plaintext: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysConfigRequest {
    pub max_version: Option<i32>,
    pub min_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysRegisterRequest {
    pub crypto_key: Option<String>,
    pub verify: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysRequest {
    pub algorithm: Option<String>,
    pub crypto_key: Option<String>,
    pub key_ring: Option<String>,
    pub labels: Option<serde_json::Map<String, serde_json::Value>>,
    pub protection_level: Option<String>,
    pub purpose: Option<String>,
    pub rotation_period: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ReencryptRequest {
    pub additional_authenticated_data: Option<String>,
    pub ciphertext: Option<String>,
    pub key_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct SignRequest {
    pub digest: Option<String>,
    pub key_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct VerifyRequest {
    pub digest: Option<String>,
    pub key_version: Option<i32>,
    pub signature: Option<String>,
}