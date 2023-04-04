use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub cf_api_addr: Option<String>,
    pub cf_api_mutual_tls_certificate: Option<String>,
    pub cf_api_mutual_tls_key: Option<String>,
    pub cf_api_trusted_certificates: Option<Vec<String>>,
    pub cf_client_id: Option<String>,
    pub cf_client_secret: Option<String>,
    pub cf_password: Option<String>,
    pub cf_username: Option<String>,
    pub identity_ca_certificates: Option<Vec<String>>,
    pub login_max_seconds_not_after: Option<i32>,
    pub login_max_seconds_not_before: Option<i32>,
    pub pcf_api_addr: Option<String>,
    pub pcf_api_trusted_certificates: Option<Vec<String>>,
    pub pcf_password: Option<String>,
    pub pcf_username: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub cf_instance_cert: String,
    pub role: String,
    pub signature: String,
    pub signing_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub bound_application_ids: Option<Vec<String>>,
    pub bound_cidrs: Option<Vec<String>>,
    pub bound_instance_ids: Option<Vec<String>>,
    pub bound_organization_ids: Option<Vec<String>>,
    pub bound_space_ids: Option<Vec<String>>,
    pub disable_ip_matching: Option<bool>,
    pub max_ttl: Option<i32>,
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