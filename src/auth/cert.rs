use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CertsRequest {
    pub allowed_common_names: Option<Vec<String>>,
    pub allowed_dns_sans: Option<Vec<String>>,
    pub allowed_email_sans: Option<Vec<String>>,
    pub allowed_metadata_extensions: Option<Vec<String>>,
    pub allowed_names: Option<Vec<String>>,
    pub allowed_organizational_units: Option<Vec<String>>,
    pub allowed_uri_sans: Option<Vec<String>>,
    pub bound_cidrs: Option<Vec<String>>,
    pub certificate: Option<String>,
    pub display_name: Option<String>,
    pub lease: Option<i32>,
    pub max_ttl: Option<i32>,
    pub ocsp_ca_certificates: Option<String>,
    pub ocsp_enabled: Option<bool>,
    pub ocsp_fail_open: Option<bool>,
    pub ocsp_query_all_servers: Option<bool>,
    pub ocsp_servers_override: Option<Vec<String>>,
    pub period: Option<i32>,
    pub policies: Option<Vec<String>>,
    pub required_extensions: Option<Vec<String>>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub disable_binding: Option<bool>,
    pub enable_identity_alias_metadata: Option<bool>,
    pub ocsp_cache_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CrlsRequest {
    pub crl: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub name: Option<String>,
}