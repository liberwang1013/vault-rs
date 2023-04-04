use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CreateOrphanRequest {
    pub display_name: Option<String>,
    pub entity_alias: Option<String>,
    pub explicit_max_ttl: Option<String>,
    pub id: Option<String>,
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub no_default_policy: Option<bool>,
    pub no_parent: Option<bool>,
    pub num_uses: Option<i32>,
    pub period: Option<String>,
    pub policies: Option<Vec<String>>,
    pub renewable: Option<bool>,
    pub role_name: Option<String>,
    pub ttl: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CreateRequest {
    pub display_name: Option<String>,
    pub entity_alias: Option<String>,
    pub explicit_max_ttl: Option<String>,
    pub id: Option<String>,
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub no_default_policy: Option<bool>,
    pub no_parent: Option<bool>,
    pub num_uses: Option<i32>,
    pub period: Option<String>,
    pub policies: Option<Vec<String>>,
    pub renewable: Option<bool>,
    pub ttl: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LookupAccessorRequest {
    pub accessor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LookupRequest {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LookupSelfRequest {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RenewAccessorRequest {
    pub accessor: Option<String>,
    pub increment: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RenewRequest {
    pub increment: Option<i32>,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RenewSelfRequest {
    pub increment: Option<i32>,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RevokeAccessorRequest {
    pub accessor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RevokeOrphanRequest {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RevokeRequest {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub allowed_entity_aliases: Option<Vec<String>>,
    pub allowed_policies: Option<Vec<String>>,
    pub allowed_policies_glob: Option<Vec<String>>,
    pub bound_cidrs: Option<Vec<String>>,
    pub disallowed_policies: Option<Vec<String>>,
    pub disallowed_policies_glob: Option<Vec<String>>,
    pub explicit_max_ttl: Option<i32>,
    pub orphan: Option<bool>,
    pub path_suffix: Option<String>,
    pub period: Option<i32>,
    pub renewable: Option<bool>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<i32>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<i32>,
    pub token_period: Option<i32>,
    pub token_type: Option<String>,
}