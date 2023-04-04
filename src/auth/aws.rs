use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigCertificateRequest {
    pub aws_public_cert: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigClientRequest {
    pub access_key: Option<String>,
    pub allowed_sts_header_values: Option<Vec<String>>,
    pub endpoint: Option<String>,
    pub iam_endpoint: Option<String>,
    pub iam_server_id_header_value: Option<String>,
    pub max_retries: Option<i32>,
    pub secret_key: Option<String>,
    pub sts_endpoint: Option<String>,
    pub sts_region: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigIdentityRequest {
    pub ec2_alias: Option<String>,
    pub ec2_metadata: Option<Vec<String>>,
    pub iam_alias: Option<String>,
    pub iam_metadata: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigStsRequest {
    pub sts_role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigTidyIdentityAccesslistRequest {
    pub disable_periodic_tidy: Option<bool>,
    pub safety_buffer: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigTidyIdentityWhitelistRequest {
    pub disable_periodic_tidy: Option<bool>,
    pub safety_buffer: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigTidyRoletagBlacklistRequest {
    pub disable_periodic_tidy: Option<bool>,
    pub safety_buffer: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigTidyRoletagDenylistRequest {
    pub disable_periodic_tidy: Option<bool>,
    pub safety_buffer: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct LoginRequest {
    pub iam_http_request_method: Option<String>,
    pub iam_request_body: Option<String>,
    pub iam_request_headers: Option<String>,
    pub iam_request_url: Option<String>,
    pub identity: Option<String>,
    pub nonce: Option<String>,
    pub pkcs7: Option<String>,
    pub role: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RoleRequest {
    pub allow_instance_migration: Option<bool>,
    pub auth_type: Option<String>,
    pub bound_account_id: Option<Vec<String>>,
    pub bound_ami_id: Option<Vec<String>>,
    pub bound_ec2_instance_id: Option<Vec<String>>,
    pub bound_iam_instance_profile_arn: Option<Vec<String>>,
    pub bound_iam_principal_arn: Option<Vec<String>>,
    pub bound_iam_role_arn: Option<Vec<String>>,
    pub bound_region: Option<Vec<String>>,
    pub bound_subnet_id: Option<Vec<String>>,
    pub bound_vpc_id: Option<Vec<String>>,
    pub disallow_reauthentication: Option<bool>,
    pub inferred_aws_region: Option<String>,
    pub inferred_entity_type: Option<String>,
    pub max_ttl: Option<i32>,
    pub period: Option<i32>,
    pub policies: Option<Vec<String>>,
    pub resolve_aws_unique_ids: Option<bool>,
    pub role_tag: Option<String>,
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
pub struct RoleTagRequest {
    pub allow_instance_migration: Option<bool>,
    pub disallow_reauthentication: Option<bool>,
    pub instance_id: Option<String>,
    pub max_ttl: Option<i32>,
    pub policies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct TidyIdentityAccesslistRequest {
    pub safety_buffer: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct TidyIdentityWhitelistRequest {
    pub safety_buffer: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct TidyRoletagBlacklistRequest {
    pub safety_buffer: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct TidyRoletagDenylistRequest {
    pub safety_buffer: Option<i32>,
}