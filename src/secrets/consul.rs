use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigAccessRequest {
    pub address: Option<String>,
    pub ca_cert: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
    pub scheme: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub consul_namespace: Option<String>,
    pub consul_policies: Option<Vec<String>>,
    pub consul_roles: Option<Vec<String>>,
    pub lease: Option<i32>,
    pub local: Option<bool>,
    pub max_ttl: Option<i32>,
    pub node_identities: Option<Vec<String>>,
    pub partition: Option<String>,
    pub policies: Option<Vec<String>>,
    pub policy: Option<String>,
    pub service_identities: Option<Vec<String>>,
    pub token_type: Option<String>,
    pub ttl: Option<i32>,
}