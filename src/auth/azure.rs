use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub environment: Option<String>,
    pub resource: Option<String>,
    pub root_password_ttl: Option<i32>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub jwt: Option<String>,
    pub resource_group_name: Option<String>,
    pub resource_id: Option<String>,
    pub role: Option<String>,
    pub subscription_id: Option<String>,
    pub vm_name: Option<String>,
    pub vmss_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub bound_group_ids: Option<Vec<String>>,
    pub bound_locations: Option<Vec<String>>,
    pub bound_resource_groups: Option<Vec<String>>,
    pub bound_scale_sets: Option<Vec<String>>,
    pub bound_service_principal_ids: Option<Vec<String>>,
    pub bound_subscription_ids: Option<Vec<String>>,
    pub max_ttl: Option<i32>,
    pub num_uses: Option<i32>,
    pub period: Option<i32>,
    pub policies: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<i32>,
    pub token_max_ttl: Option<i32>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<i32>,
    pub token_period: Option<i32>,
    pub token_policies: Option<Vec<String>>,
    pub token_ttl: Option<i32>,
    pub token_type: Option<String>,
    pub ttl: Option<i32>,
}