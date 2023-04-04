use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub allowed_roles: Option<Vec<String>>,
    pub password_policy: Option<String>,
    pub plugin_name: Option<String>,
    pub plugin_version: Option<String>,
    pub root_rotation_statements: Option<Vec<String>>,
    pub verify_connection: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub creation_statements: Option<Vec<String>>,
    pub credential_config: Option<serde_json::Map<String, serde_json::Value>>,
    pub credential_type: Option<String>,
    pub db_name: Option<String>,
    pub default_ttl: Option<i32>,
    pub max_ttl: Option<i32>,
    pub renew_statements: Option<Vec<String>>,
    pub revocation_statements: Option<Vec<String>>,
    pub rollback_statements: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct StaticRolesRequest {
    pub credential_config: Option<serde_json::Map<String, serde_json::Value>>,
    pub credential_type: Option<String>,
    pub db_name: Option<String>,
    pub rotation_period: Option<i32>,
    pub rotation_statements: Option<Vec<String>>,
    pub username: Option<String>,
}