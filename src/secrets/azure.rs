use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub environment: Option<String>,
    pub password_policy: Option<String>,
    pub root_password_ttl: Option<i32>,
    pub subscription_id: Option<String>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub application_object_id: Option<String>,
    pub azure_groups: Option<String>,
    pub azure_roles: Option<String>,
    pub max_ttl: Option<i32>,
    pub permanently_delete: Option<bool>,
    pub persist_app: Option<bool>,
    pub ttl: Option<i32>,
}