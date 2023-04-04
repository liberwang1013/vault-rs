use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub anonymous_group_search: Option<bool>,
    pub binddn: Option<String>,
    pub bindpass: Option<String>,
    pub case_sensitive_names: Option<bool>,
    pub certificate: Option<String>,
    pub client_tls_cert: Option<String>,
    pub client_tls_key: Option<String>,
    pub deny_null_bind: Option<bool>,
    pub discoverdn: Option<bool>,
    pub groupattr: Option<String>,
    pub groupdn: Option<String>,
    pub groupfilter: Option<String>,
    pub insecure_tls: Option<bool>,
    pub length: Option<i32>,
    pub max_ttl: Option<i32>,
    pub password_policy: Option<String>,
    pub request_timeout: Option<i32>,
    pub schema: Option<String>,
    pub starttls: Option<bool>,
    pub tls_max_version: Option<String>,
    pub tls_min_version: Option<String>,
    pub ttl: Option<i32>,
    pub upndomain: Option<String>,
    pub url: Option<String>,
    pub use_pre111_group_cn_behavior: Option<bool>,
    pub use_token_groups: Option<bool>,
    pub userattr: Option<String>,
    pub userdn: Option<String>,
    pub userfilter: Option<String>,
    pub username_as_alias: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LibraryCheckInRequest {
    pub service_account_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LibraryCheckOutRequest {
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LibraryManageCheckInRequest {
    pub service_account_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LibraryRequest {
    pub disable_check_in_enforcement: Option<bool>,
    pub max_ttl: Option<i32>,
    pub service_account_names: Option<Vec<String>>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub creation_ldif: String,
    pub default_ttl: Option<i32>,
    pub deletion_ldif: String,
    pub max_ttl: Option<i32>,
    pub rollback_ldif: Option<String>,
    pub username_template: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct StaticRoleRequest {
    pub dn: Option<String>,
    pub rotation_period: Option<i32>,
    pub username: Option<String>,
}