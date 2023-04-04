use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub role_id: Option<String>,
    pub secret_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginResponse {
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleBindSecretIdRequest {
    pub bind_secret_id: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleBindSecretIdResponse {
    pub bind_secret_id: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleBoundCidrListRequest {
    pub bound_cidr_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleBoundCidrListResponse {
    pub bound_cidr_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleCustomSecretIdRequest {
    pub cidr_list: Option<Vec<String>>,
    pub metadata: Option<String>,
    pub num_uses: Option<i32>,
    pub secret_id: Option<String>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleCustomSecretIdResponse {
    pub secret_id: Option<String>,
    pub secret_id_accessor: Option<String>,
    pub secret_id_num_uses: Option<i32>,
    pub secret_id_ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleLocalSecretIdsResponse {
    pub local_secret_ids: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolePeriodRequest {
    pub period: Option<i32>,
    pub token_period: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolePeriodResponse {
    pub period: Option<i32>,
    pub token_period: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolePoliciesRequest {
    pub policies: Option<Vec<String>>,
    pub token_policies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolePoliciesResponse {
    pub policies: Option<Vec<String>>,
    pub token_policies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub bind_secret_id: Option<bool>,
    pub bound_cidr_list: Option<Vec<String>>,
    pub local_secret_ids: Option<bool>,
    pub period: Option<i32>,
    pub policies: Option<Vec<String>>,
    pub role_id: Option<String>,
    pub secret_id_bound_cidrs: Option<Vec<String>>,
    pub secret_id_num_uses: Option<i32>,
    pub secret_id_ttl: Option<i32>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<i32>,
    pub token_max_ttl: Option<i32>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<i32>,
    pub token_period: Option<i32>,
    pub token_policies: Option<Vec<String>>,
    pub token_ttl: Option<i32>,
    pub token_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleResponse {
    pub keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRoleIdRequest {
    pub role_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRoleIdResponse {
    pub role_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdAccessorDestroyRequest {
    pub secret_id_accessor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdAccessorLookupRequest {
    pub secret_id_accessor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdAccessorLookupResponse {
    pub cidr_list: Option<Vec<String>>,
    pub creation_time: Option<String>,
    pub expiration_time: Option<String>,
    pub last_updated_time: Option<String>,
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub secret_id_accessor: Option<String>,
    pub secret_id_num_uses: Option<i32>,
    pub secret_id_ttl: Option<i32>,
    pub token_bound_cidrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdBoundCidrsRequest {
    pub secret_id_bound_cidrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdBoundCidrsResponse {
    pub secret_id_bound_cidrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdDestroyRequest {
    pub secret_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdLookupRequest {
    pub secret_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdLookupResponse {
    pub cidr_list: Option<Vec<String>>,
    pub creation_time: Option<String>,
    pub expiration_time: Option<String>,
    pub last_updated_time: Option<String>,
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    pub secret_id_accessor: Option<String>,
    pub secret_id_num_uses: Option<i32>,
    pub secret_id_ttl: Option<i32>,
    pub token_bound_cidrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdNumUsesRequest {
    pub secret_id_num_uses: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdNumUsesResponse {
    pub secret_id_num_uses: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdRequest {
    pub cidr_list: Option<Vec<String>>,
    pub metadata: Option<String>,
    pub num_uses: Option<i32>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdResponse {
    pub keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdTtlRequest {
    pub secret_id_ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleSecretIdTtlResponse {
    pub secret_id_ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleTokenBoundCidrsRequest {
    pub token_bound_cidrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleTokenBoundCidrsResponse {
    pub token_bound_cidrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleTokenMaxTtlRequest {
    pub token_max_ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleTokenMaxTtlResponse {
    pub token_max_ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleTokenNumUsesRequest {
    pub token_num_uses: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleTokenNumUsesResponse {
    pub token_num_uses: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleTokenTtlRequest {
    pub token_ttl: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleTokenTtlResponse {
    pub token_ttl: Option<i32>,
}