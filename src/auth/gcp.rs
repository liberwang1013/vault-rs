use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub credentials: Option<String>,
    pub custom_endpoint: Option<serde_json::Map<String, serde_json::Value>>,
    pub gce_alias: Option<String>,
    pub gce_metadata: Option<Vec<String>>,
    pub google_certs_endpoint: Option<String>,
    pub iam_alias: Option<String>,
    pub iam_metadata: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub jwt: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleLabelsRequest {
    pub add: Option<Vec<String>>,
    pub remove: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub add_group_aliases: Option<bool>,
    pub allow_gce_inference: Option<bool>,
    pub bound_instance_group: Option<String>,
    pub bound_instance_groups: Option<Vec<String>>,
    pub bound_labels: Option<Vec<String>>,
    pub bound_projects: Option<Vec<String>>,
    pub bound_region: Option<String>,
    pub bound_regions: Option<Vec<String>>,
    pub bound_service_accounts: Option<Vec<String>>,
    pub bound_zone: Option<String>,
    pub bound_zones: Option<Vec<String>>,
    pub max_jwt_exp: Option<i32>,
    pub max_ttl: Option<i32>,
    pub period: Option<i32>,
    pub policies: Option<Vec<String>>,
    pub project_id: Option<String>,
    pub service_accounts: Option<Vec<String>>,
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
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleServiceAccountsRequest {
    pub add: Option<Vec<String>>,
    pub remove: Option<Vec<String>>,
}