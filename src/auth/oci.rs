use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub home_tenancy_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub request_headers: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub ocid_list: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<i32>,
    pub token_max_ttl: Option<i32>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<i32>,
    pub token_period: Option<i32>,
    pub token_policies: Option<Vec<String>>,
    pub token_ttl: Option<i32>,
    pub token_type: Option<String>,
}