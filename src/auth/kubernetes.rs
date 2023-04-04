use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub disable_iss_validation: Option<bool>,
    pub disable_local_ca_jwt: Option<bool>,
    pub issuer: Option<String>,
    pub kubernetes_ca_cert: Option<String>,
    pub kubernetes_host: Option<String>,
    pub pem_keys: Option<Vec<String>>,
    pub token_reviewer_jwt: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub jwt: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub alias_name_source: Option<String>,
    pub audience: Option<String>,
    pub bound_cidrs: Option<Vec<String>>,
    pub bound_service_account_names: Option<Vec<String>>,
    pub bound_service_account_namespaces: Option<Vec<String>>,
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