use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CacheConfigRequest {
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigKeysRequest {
    pub disable_upsert: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct DatakeyRequest {
    pub bits: Option<i32>,
    pub context: Option<String>,
    pub key_version: Option<i32>,
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct DecryptRequest {
    pub associated_data: Option<String>,
    pub batch_input: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
    pub ciphertext: Option<String>,
    pub context: Option<String>,
    pub nonce: Option<String>,
    pub partial_failure_response_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct EncryptRequest {
    pub associated_data: Option<String>,
    pub batch_input: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
    pub context: Option<String>,
    pub convergent_encryption: Option<bool>,
    pub key_version: Option<i32>,
    pub nonce: Option<String>,
    pub partial_failure_response_code: Option<i32>,
    pub plaintext: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct HashRequest {
    pub algorithm: Option<String>,
    pub format: Option<String>,
    pub input: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct HmacRequest {
    pub algorithm: Option<String>,
    pub batch_input: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
    pub input: Option<String>,
    pub key_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysConfigRequest {
    pub allow_plaintext_backup: Option<bool>,
    pub auto_rotate_period: Option<i32>,
    pub deletion_allowed: Option<bool>,
    pub exportable: Option<bool>,
    pub min_decryption_version: Option<i32>,
    pub min_encryption_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysImportRequest {
    pub allow_plaintext_backup: Option<bool>,
    pub allow_rotation: Option<bool>,
    pub auto_rotate_period: Option<i32>,
    pub ciphertext: Option<String>,
    pub context: Option<String>,
    pub derived: Option<bool>,
    pub exportable: Option<bool>,
    pub hash_function: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysImportVersionRequest {
    pub ciphertext: Option<String>,
    pub hash_function: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysRequest {
    pub allow_plaintext_backup: Option<bool>,
    pub auto_rotate_period: Option<i32>,
    pub context: Option<String>,
    pub convergent_encryption: Option<bool>,
    pub derived: Option<bool>,
    pub exportable: Option<bool>,
    pub key_size: Option<i32>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysRotateRequest {
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysTrimRequest {
    pub min_available_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RandomRequest {
    pub bytes: Option<i32>,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RestoreRequest {
    pub backup: Option<String>,
    pub force: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RewrapRequest {
    pub batch_input: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
    pub ciphertext: Option<String>,
    pub context: Option<String>,
    pub key_version: Option<i32>,
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct SignRequest {
    pub algorithm: Option<String>,
    pub batch_input: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
    pub context: Option<String>,
    pub hash_algorithm: Option<String>,
    pub input: Option<String>,
    pub key_version: Option<i32>,
    pub marshaling_algorithm: Option<String>,
    pub prehashed: Option<bool>,
    pub salt_length: Option<String>,
    pub signature_algorithm: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct VerifyRequest {
    pub algorithm: Option<String>,
    pub batch_input: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
    pub context: Option<String>,
    pub hash_algorithm: Option<String>,
    pub hmac: Option<String>,
    pub input: Option<String>,
    pub marshaling_algorithm: Option<String>,
    pub prehashed: Option<bool>,
    pub salt_length: Option<String>,
    pub signature: Option<String>,
    pub signature_algorithm: Option<String>,
}