use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub cidr_blocks: Option<Vec<String>>,
    pub ip_addresses: Option<Vec<String>>,
    pub max_ttl: Option<i32>,
    pub organization_id: Option<String>,
    pub project_id: Option<String>,
    pub project_roles: Option<Vec<String>>,
    pub roles: Vec<String>,
    pub ttl: Option<i32>,
}