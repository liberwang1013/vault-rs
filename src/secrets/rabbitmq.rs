use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigConnectionRequest {
    pub connection_uri: Option<String>,
    pub password: Option<String>,
    pub password_policy: Option<String>,
    pub username: Option<String>,
    pub username_template: Option<String>,
    pub verify_connection: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigLeaseRequest {
    pub max_ttl: Option<i32>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub tags: Option<String>,
    pub vhost_topics: Option<String>,
    pub vhosts: Option<String>,
}