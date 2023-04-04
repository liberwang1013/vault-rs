use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigLeaseRequest {
    pub lease: Option<String>,
    pub lease_max: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRootRequest {
    pub access_key: Option<String>,
    pub iam_endpoint: Option<String>,
    pub max_retries: Option<i32>,
    pub region: Option<String>,
    pub secret_key: Option<String>,
    pub sts_endpoint: Option<String>,
    pub username_template: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CredsRequest {
    pub role_arn: Option<String>,
    pub role_session_name: Option<String>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub arn: Option<String>,
    pub credential_type: Option<String>,
    pub default_sts_ttl: Option<i32>,
    pub iam_groups: Option<Vec<String>>,
    pub iam_tags: Option<serde_json::Map<String, serde_json::Value>>,
    pub max_sts_ttl: Option<i32>,
    pub permissions_boundary_arn: Option<String>,
    pub policy: Option<String>,
    pub policy_arns: Option<Vec<String>>,
    pub policy_document: Option<String>,
    pub role_arns: Option<Vec<String>>,
    pub user_path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct StsRequest {
    pub role_arn: Option<String>,
    pub role_session_name: Option<String>,
    pub ttl: Option<i32>,
}