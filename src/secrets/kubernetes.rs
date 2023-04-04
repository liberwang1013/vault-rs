use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigRequest {
    pub disable_local_ca_jwt: Option<bool>,
    pub kubernetes_ca_cert: Option<String>,
    pub kubernetes_host: Option<String>,
    pub service_account_jwt: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CredsRequest {
    pub cluster_role_binding: Option<bool>,
    pub kubernetes_namespace: String,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub allowed_kubernetes_namespace_selector: Option<String>,
    pub allowed_kubernetes_namespaces: Option<Vec<String>>,
    pub extra_annotations: Option<serde_json::Map<String, serde_json::Value>>,
    pub extra_labels: Option<serde_json::Map<String, serde_json::Value>>,
    pub generated_role_rules: Option<String>,
    pub kubernetes_role_name: Option<String>,
    pub kubernetes_role_type: Option<String>,
    pub name_template: Option<String>,
    pub service_account_name: Option<String>,
    pub token_default_ttl: Option<i32>,
    pub token_max_ttl: Option<i32>,
}