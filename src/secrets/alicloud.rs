use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub inline_policies: Option<String>,
    pub max_ttl: Option<i32>,
    pub remote_policies: Option<Vec<String>>,
    pub role_arn: Option<String>,
    pub ttl: Option<i32>,
}