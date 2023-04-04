use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct AuditHashRequest {
    pub input: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct AuditHashResponse {
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct AuditRequest {
    pub description: Option<String>,
    pub local: Option<bool>,
    pub options: Option<serde_json::Map<String, serde_json::Value>>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct AuthRequest {
    pub config: Option<serde_json::Map<String, serde_json::Value>>,
    pub description: Option<String>,
    pub external_entropy_access: Option<bool>,
    pub local: Option<bool>,
    pub options: Option<serde_json::Map<String, serde_json::Value>>,
    pub plugin_name: Option<String>,
    pub plugin_version: Option<String>,
    pub seal_wrap: Option<bool>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct AuthTuneRequest {
    pub allowed_response_headers: Option<Vec<String>>,
    pub audit_non_hmac_request_keys: Option<Vec<String>>,
    pub audit_non_hmac_response_keys: Option<Vec<String>>,
    pub default_lease_ttl: Option<String>,
    pub description: Option<String>,
    pub listing_visibility: Option<String>,
    pub max_lease_ttl: Option<String>,
    pub options: Option<serde_json::Map<String, serde_json::Value>>,
    pub passthrough_request_headers: Option<Vec<String>>,
    pub plugin_version: Option<String>,
    pub token_type: Option<String>,
    pub user_lockout_config: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CapabilitiesAccessorRequest {
    pub accessor: Option<String>,
    pub path: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CapabilitiesRequest {
    pub path: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CapabilitiesSelfRequest {
    pub path: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigAuditingRequestHeadersRequest {
    pub hmac: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigAuditingRequestHeadersResponse {
    pub headers: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigCorsRequest {
    pub allowed_headers: Option<Vec<String>>,
    pub allowed_origins: Option<Vec<String>>,
    pub enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigUiHeadersRequest {
    pub multivalue: Option<bool>,
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct GenerateRootAttemptRequest {
    pub pgp_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct GenerateRootRequest {
    pub pgp_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct GenerateRootUpdateRequest {
    pub key: Option<String>,
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct InitRequest {
    pub pgp_keys: Option<Vec<String>>,
    pub recovery_pgp_keys: Option<Vec<String>>,
    pub recovery_shares: Option<i32>,
    pub recovery_threshold: Option<i32>,
    pub root_token_pgp_key: Option<String>,
    pub secret_shares: Option<i32>,
    pub secret_threshold: Option<i32>,
    pub stored_shares: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct InternalCountersConfigRequest {
    pub default_report_months: Option<i32>,
    pub enabled: Option<String>,
    pub retention_months: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct InternalSpecsOpenapiRequest {
    pub context: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LeasesLookupRequest {
    pub lease_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LeasesRenewLeaseRequest {
    pub increment: Option<i32>,
    pub lease_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LeasesRenewRequest {
    pub increment: Option<i32>,
    pub lease_id: Option<String>,
    pub url_lease_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LeasesRevokeLeaseRequest {
    pub lease_id: Option<String>,
    pub sync: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LeasesRevokePrefixRequest {
    pub sync: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LeasesRevokeRequest {
    pub lease_id: Option<String>,
    pub sync: Option<bool>,
    pub url_lease_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoggersRequest {
    pub level: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct MfaValidateRequest {
    pub mfa_payload: serde_json::Map<String, serde_json::Value>,
    pub mfa_request_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct MountsRequest {
    pub config: Option<serde_json::Map<String, serde_json::Value>>,
    pub description: Option<String>,
    pub external_entropy_access: Option<bool>,
    pub local: Option<bool>,
    pub options: Option<serde_json::Map<String, serde_json::Value>>,
    pub plugin_name: Option<String>,
    pub plugin_version: Option<String>,
    pub seal_wrap: Option<bool>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct MountsTuneRequest {
    pub allowed_managed_keys: Option<Vec<String>>,
    pub allowed_response_headers: Option<Vec<String>>,
    pub audit_non_hmac_request_keys: Option<Vec<String>>,
    pub audit_non_hmac_response_keys: Option<Vec<String>>,
    pub default_lease_ttl: Option<String>,
    pub description: Option<String>,
    pub listing_visibility: Option<String>,
    pub max_lease_ttl: Option<String>,
    pub options: Option<serde_json::Map<String, serde_json::Value>>,
    pub passthrough_request_headers: Option<Vec<String>>,
    pub plugin_version: Option<String>,
    pub token_type: Option<String>,
    pub user_lockout_config: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct PluginsCatalogRequest {
    pub args: Option<Vec<String>>,
    pub command: Option<String>,
    pub env: Option<Vec<String>>,
    pub sha256: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct PluginsReloadBackendRequest {
    pub mounts: Option<Vec<String>>,
    pub plugin: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct PoliciesAclRequest {
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct PoliciesPasswordRequest {
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct PolicyRequest {
    pub policy: Option<String>,
    pub rules: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct QuotasConfigRequest {
    pub enable_rate_limit_audit_logging: Option<bool>,
    pub enable_rate_limit_response_headers: Option<bool>,
    pub rate_limit_exempt_paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct QuotasRateLimitRequest {
    pub block_interval: Option<i32>,
    pub interval: Option<i32>,
    pub path: Option<String>,
    pub rate: Option<f32>,
    pub role: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RawRequest {
    pub compressed: Option<bool>,
    pub compression_type: Option<String>,
    pub encoding: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RekeyInitRequest {
    pub backup: Option<bool>,
    pub pgp_keys: Option<Vec<String>>,
    pub require_verification: Option<bool>,
    pub secret_shares: Option<i32>,
    pub secret_threshold: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RekeyUpdateRequest {
    pub key: Option<String>,
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RekeyVerifyRequest {
    pub key: Option<String>,
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RemountRequest {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RenewLeaseRequest {
    pub increment: Option<i32>,
    pub lease_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RenewRequest {
    pub increment: Option<i32>,
    pub lease_id: Option<String>,
    pub url_lease_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RevokeLeaseRequest {
    pub lease_id: Option<String>,
    pub sync: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RevokePrefixRequest {
    pub sync: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RevokeRequest {
    pub lease_id: Option<String>,
    pub sync: Option<bool>,
    pub url_lease_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RotateConfigRequest {
    pub enabled: Option<bool>,
    pub interval: Option<i32>,
    pub max_operations: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ToolsHashRequest {
    pub algorithm: Option<String>,
    pub format: Option<String>,
    pub input: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ToolsRandomRequest {
    pub bytes: Option<i32>,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct UnsealRequest {
    pub key: Option<String>,
    pub reset: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct WrappingLookupRequest {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct WrappingRewrapRequest {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct WrappingUnwrapRequest {
    pub token: Option<String>,
}