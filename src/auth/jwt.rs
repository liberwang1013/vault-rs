use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub bound_issuer: Option<String>,
    pub default_role: Option<String>,
    pub jwks_ca_pem: Option<String>,
    pub jwks_url: Option<String>,
    pub jwt_supported_algs: Option<Vec<String>>,
    pub jwt_validation_pubkeys: Option<Vec<String>>,
    pub namespace_in_state: Option<bool>,
    pub oidc_client_id: Option<String>,
    pub oidc_client_secret: Option<String>,
    pub oidc_discovery_ca_pem: Option<String>,
    pub oidc_discovery_url: Option<String>,
    pub oidc_response_mode: Option<String>,
    pub oidc_response_types: Option<Vec<String>>,
    pub provider_config: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub jwt: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct OidcAuthUrlRequest {
    pub client_nonce: Option<String>,
    pub redirect_uri: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct OidcCallbackRequest {
    pub client_nonce: Option<String>,
    pub code: Option<String>,
    pub id_token: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub allowed_redirect_uris: Option<Vec<String>>,
    pub bound_audiences: Option<Vec<String>>,
    pub bound_cidrs: Option<Vec<String>>,
    pub bound_claims: Option<serde_json::Map<String, serde_json::Value>>,
    pub bound_claims_type: Option<String>,
    pub bound_subject: Option<String>,
    pub claim_mappings: Option<serde_json::Map<String, serde_json::Value>>,
    pub clock_skew_leeway: Option<i32>,
    pub expiration_leeway: Option<i32>,
    pub groups_claim: Option<String>,
    pub max_age: Option<i32>,
    pub max_ttl: Option<i32>,
    pub not_before_leeway: Option<i32>,
    pub num_uses: Option<i32>,
    pub oidc_scopes: Option<Vec<String>>,
    pub period: Option<i32>,
    pub policies: Option<Vec<String>>,
    pub role_type: Option<String>,
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
    pub user_claim: Option<String>,
    pub user_claim_json_pointer: Option<bool>,
    pub verbose_oidc_logging: Option<bool>,
}