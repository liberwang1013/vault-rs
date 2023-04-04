use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub address: Option<String>,
    pub base_path: Option<String>,
    pub token: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub max_ttl: Option<i32>,
    pub organization: Option<String>,
    pub team_id: Option<String>,
    pub ttl: Option<i32>,
    pub user_id: Option<String>,
}