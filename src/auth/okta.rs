use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub api_token: Option<String>,
    pub base_url: Option<String>,
    pub bypass_okta_mfa: Option<bool>,
    pub max_ttl: Option<i32>,
    pub org_name: Option<String>,
    pub organization: Option<String>,
    pub production: Option<bool>,
    pub token: Option<String>,
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
pub struct GroupsRequest {
    pub policies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub nonce: Option<String>,
    pub password: Option<String>,
    pub provider: Option<String>,
    pub totp: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct UsersRequest {
    pub groups: Option<Vec<String>>,
    pub policies: Option<Vec<String>>,
}