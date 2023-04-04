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
    pub request_timeout: Option<i32>,
    pub starttls: Option<bool>,
    pub tls_max_version: Option<String>,
    pub tls_min_version: Option<String>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<i32>,
    pub token_max_ttl: Option<i32>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<i32>,
    pub token_period: Option<i32>,
    pub token_policies: Option<Vec<String>>,
    pub token_ttl: Option<i32>,
    pub token_type: Option<String>,
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
pub struct GroupsRequest {
    pub policies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub password: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct UsersRequest {
    pub groups: Option<Vec<String>>,
    pub policies: Option<Vec<String>>,
}