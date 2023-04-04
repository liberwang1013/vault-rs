use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigCaRequest {
    pub generate_signing_key: Option<bool>,
    pub key_bits: Option<i32>,
    pub key_type: Option<String>,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigZeroaddressRequest {
    pub roles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CredsRequest {
    pub ip: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssueRequest {
    pub cert_type: Option<String>,
    pub critical_options: Option<serde_json::Map<String, serde_json::Value>>,
    pub extensions: Option<serde_json::Map<String, serde_json::Value>>,
    pub key_bits: Option<i32>,
    pub key_id: Option<String>,
    pub key_type: Option<String>,
    pub ttl: Option<i32>,
    pub valid_principals: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LookupRequest {
    pub ip: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub algorithm_signer: Option<String>,
    pub allow_bare_domains: Option<bool>,
    pub allow_host_certificates: Option<bool>,
    pub allow_subdomains: Option<bool>,
    pub allow_user_certificates: Option<bool>,
    pub allow_user_key_ids: Option<bool>,
    pub allowed_critical_options: Option<String>,
    pub allowed_domains: Option<String>,
    pub allowed_domains_template: Option<bool>,
    pub allowed_extensions: Option<String>,
    pub allowed_user_key_lengths: Option<serde_json::Map<String, serde_json::Value>>,
    pub allowed_users: Option<String>,
    pub allowed_users_template: Option<bool>,
    pub cidr_list: Option<String>,
    pub default_critical_options: Option<serde_json::Map<String, serde_json::Value>>,
    pub default_extensions: Option<serde_json::Map<String, serde_json::Value>>,
    pub default_extensions_template: Option<bool>,
    pub default_user: Option<String>,
    pub default_user_template: Option<bool>,
    pub exclude_cidr_list: Option<String>,
    pub key_id_format: Option<String>,
    pub key_type: Option<String>,
    pub max_ttl: Option<i32>,
    pub not_before_duration: Option<i32>,
    pub port: Option<i32>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct SignRequest {
    pub cert_type: Option<String>,
    pub critical_options: Option<serde_json::Map<String, serde_json::Value>>,
    pub extensions: Option<serde_json::Map<String, serde_json::Value>>,
    pub key_id: Option<String>,
    pub public_key: Option<String>,
    pub ttl: Option<i32>,
    pub valid_principals: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct VerifyRequest {
    pub otp: Option<String>,
}