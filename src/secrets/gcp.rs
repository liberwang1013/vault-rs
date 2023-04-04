use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub credentials: Option<String>,
    pub max_ttl: Option<i32>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ImpersonatedAccountRequest {
    pub service_account_email: Option<String>,
    pub token_scopes: Option<Vec<String>>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeyRequest {
    pub key_algorithm: Option<String>,
    pub key_type: Option<String>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesetKeyRequest {
    pub key_algorithm: Option<String>,
    pub key_type: Option<String>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesetRequest {
    pub bindings: Option<String>,
    pub project: Option<String>,
    pub secret_type: Option<String>,
    pub token_scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct StaticAccountKeyRequest {
    pub key_algorithm: Option<String>,
    pub key_type: Option<String>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct StaticAccountRequest {
    pub bindings: Option<String>,
    pub secret_type: Option<String>,
    pub service_account_email: Option<String>,
    pub token_scopes: Option<Vec<String>>,
}