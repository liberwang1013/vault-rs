use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigAutoTidyRequest {
    pub enabled: Option<bool>,
    pub interval_duration: Option<i32>,
    pub issuer_safety_buffer: Option<i32>,
    pub pause_duration: Option<String>,
    pub revocation_queue_safety_buffer: Option<i32>,
    pub safety_buffer: Option<i32>,
    pub tidy_cert_store: Option<bool>,
    pub tidy_cross_cluster_revoked_certs: Option<bool>,
    pub tidy_expired_issuers: Option<bool>,
    pub tidy_move_legacy_ca_bundle: Option<bool>,
    pub tidy_revocation_list: Option<bool>,
    pub tidy_revocation_queue: Option<bool>,
    pub tidy_revoked_cert_issuer_associations: Option<bool>,
    pub tidy_revoked_certs: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigCaRequest {
    pub pem_bundle: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigClusterRequest {
    pub aia_path: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigCrlRequest {
    pub auto_rebuild: Option<bool>,
    pub auto_rebuild_grace_period: Option<String>,
    pub cross_cluster_revocation: Option<bool>,
    pub delta_rebuild_interval: Option<String>,
    pub disable: Option<bool>,
    pub enable_delta: Option<bool>,
    pub expiry: Option<String>,
    pub ocsp_disable: Option<bool>,
    pub ocsp_expiry: Option<String>,
    pub unified_crl: Option<bool>,
    pub unified_crl_on_existing_paths: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigIssuersRequest {
    pub default: Option<String>,
    pub default_follows_latest_issuer: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigKeysRequest {
    pub default: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct ConfigUrlsRequest {
    pub crl_distribution_points: Option<Vec<String>>,
    pub enable_templating: Option<bool>,
    pub issuing_certificates: Option<Vec<String>>,
    pub ocsp_servers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IntermediateCrossSignRequest {
    pub add_basic_constraints: Option<bool>,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub exported: Option<String>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub key_bits: Option<i32>,
    pub key_name: Option<String>,
    pub key_ref: Option<String>,
    pub key_type: Option<String>,
    pub locality: Option<Vec<String>>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
    pub not_after: Option<String>,
    pub not_before_duration: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IntermediateGenerateRequest {
    pub add_basic_constraints: Option<bool>,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub key_bits: Option<i32>,
    pub key_name: Option<String>,
    pub key_ref: Option<String>,
    pub key_type: Option<String>,
    pub locality: Option<Vec<String>>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
    pub not_after: Option<String>,
    pub not_before_duration: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IntermediateSetSignedRequest {
    pub certificate: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssueRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub issuer_ref: Option<String>,
    pub not_after: Option<String>,
    pub other_sans: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub remove_roots_from_chain: Option<bool>,
    pub serial_number: Option<String>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerDerRequest {
    pub crl_distribution_points: Option<Vec<String>>,
    pub enable_aia_url_templating: Option<bool>,
    pub issuer_name: Option<String>,
    pub issuing_certificates: Option<Vec<String>>,
    pub leaf_not_after_behavior: Option<String>,
    pub manual_chain: Option<Vec<String>>,
    pub ocsp_servers: Option<Vec<String>>,
    pub revocation_signature_algorithm: Option<String>,
    pub usage: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerIssueRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub not_after: Option<String>,
    pub other_sans: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub remove_roots_from_chain: Option<bool>,
    pub serial_number: Option<String>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerJsonRequest {
    pub crl_distribution_points: Option<Vec<String>>,
    pub enable_aia_url_templating: Option<bool>,
    pub issuer_name: Option<String>,
    pub issuing_certificates: Option<Vec<String>>,
    pub leaf_not_after_behavior: Option<String>,
    pub manual_chain: Option<Vec<String>>,
    pub ocsp_servers: Option<Vec<String>>,
    pub revocation_signature_algorithm: Option<String>,
    pub usage: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerPemRequest {
    pub crl_distribution_points: Option<Vec<String>>,
    pub enable_aia_url_templating: Option<bool>,
    pub issuer_name: Option<String>,
    pub issuing_certificates: Option<Vec<String>>,
    pub leaf_not_after_behavior: Option<String>,
    pub manual_chain: Option<Vec<String>>,
    pub ocsp_servers: Option<Vec<String>>,
    pub revocation_signature_algorithm: Option<String>,
    pub usage: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerRequest {
    pub crl_distribution_points: Option<Vec<String>>,
    pub enable_aia_url_templating: Option<bool>,
    pub issuer_name: Option<String>,
    pub issuing_certificates: Option<Vec<String>>,
    pub leaf_not_after_behavior: Option<String>,
    pub manual_chain: Option<Vec<String>>,
    pub ocsp_servers: Option<Vec<String>>,
    pub revocation_signature_algorithm: Option<String>,
    pub usage: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerResignCrlsRequest {
    pub crl_number: Option<i32>,
    pub crls: Option<Vec<String>>,
    pub delta_crl_base_number: Option<i32>,
    pub format: Option<String>,
    pub next_update: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerSignIntermediateRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub csr: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub issuer_name: Option<String>,
    pub locality: Option<Vec<String>>,
    pub max_path_length: Option<i32>,
    pub not_after: Option<String>,
    pub not_before_duration: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub permitted_dns_domains: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub skid: Option<String>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub use_csr_values: Option<bool>,
    pub use_pss: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerSignRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub csr: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub not_after: Option<String>,
    pub other_sans: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub remove_roots_from_chain: Option<bool>,
    pub serial_number: Option<String>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerSignRevocationListRequest {
    pub crl_number: Option<i32>,
    pub delta_crl_base_number: Option<i32>,
    pub extensions: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
    pub format: Option<String>,
    pub next_update: Option<String>,
    pub revoked_certs: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerSignSelfIssuedRequest {
    pub certificate: Option<String>,
    pub require_matching_certificate_algorithms: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuerSignVerbatimRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub csr: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub ext_key_usage: Option<Vec<String>>,
    pub ext_key_usage_oids: Option<Vec<String>>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub key_usage: Option<Vec<String>>,
    pub not_after: Option<String>,
    pub other_sans: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub remove_roots_from_chain: Option<bool>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub use_pss: Option<bool>,
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuersGenerateIntermediateRequest {
    pub add_basic_constraints: Option<bool>,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub key_bits: Option<i32>,
    pub key_name: Option<String>,
    pub key_ref: Option<String>,
    pub key_type: Option<String>,
    pub locality: Option<Vec<String>>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
    pub not_after: Option<String>,
    pub not_before_duration: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuersGenerateRootRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub issuer_name: Option<String>,
    pub key_bits: Option<i32>,
    pub key_name: Option<String>,
    pub key_ref: Option<String>,
    pub key_type: Option<String>,
    pub locality: Option<Vec<String>>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
    pub max_path_length: Option<i32>,
    pub not_after: Option<String>,
    pub not_before_duration: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub permitted_dns_domains: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub use_pss: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuersImportBundleRequest {
    pub pem_bundle: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct IssuersImportCertRequest {
    pub pem_bundle: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeyRequest {
    pub key_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysGenerateExportedRequest {
    pub key_bits: Option<i32>,
    pub key_name: Option<String>,
    pub key_type: Option<String>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysGenerateInternalRequest {
    pub key_bits: Option<i32>,
    pub key_name: Option<String>,
    pub key_type: Option<String>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysGenerateKmsRequest {
    pub key_bits: Option<i32>,
    pub key_name: Option<String>,
    pub key_type: Option<String>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysImportRequest {
    pub key_name: Option<String>,
    pub pem_bundle: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RevokeRequest {
    pub certificate: Option<String>,
    pub serial_number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RevokeWithKeyRequest {
    pub certificate: Option<String>,
    pub private_key: Option<String>,
    pub serial_number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RolesRequest {
    pub allow_any_name: Option<bool>,
    pub allow_bare_domains: Option<bool>,
    pub allow_glob_domains: Option<bool>,
    pub allow_ip_sans: Option<bool>,
    pub allow_localhost: Option<bool>,
    pub allow_subdomains: Option<bool>,
    pub allow_wildcard_certificates: Option<bool>,
    pub allowed_domains: Option<Vec<String>>,
    pub allowed_domains_template: Option<bool>,
    pub allowed_other_sans: Option<Vec<String>>,
    pub allowed_serial_numbers: Option<Vec<String>>,
    pub allowed_uri_sans: Option<Vec<String>>,
    pub allowed_uri_sans_template: Option<bool>,
    pub allowed_user_ids: Option<Vec<String>>,
    pub backend: Option<String>,
    pub basic_constraints_valid_for_non_ca: Option<bool>,
    pub client_flag: Option<bool>,
    pub cn_validations: Option<Vec<String>>,
    pub code_signing_flag: Option<bool>,
    pub country: Option<Vec<String>>,
    pub email_protection_flag: Option<bool>,
    pub enforce_hostnames: Option<bool>,
    pub ext_key_usage: Option<Vec<String>>,
    pub ext_key_usage_oids: Option<Vec<String>>,
    pub generate_lease: Option<bool>,
    pub issuer_ref: Option<String>,
    pub key_bits: Option<i32>,
    pub key_type: Option<String>,
    pub key_usage: Option<Vec<String>>,
    pub locality: Option<Vec<String>>,
    pub max_ttl: Option<i32>,
    pub no_store: Option<bool>,
    pub not_after: Option<String>,
    pub not_before_duration: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub policy_identifiers: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub province: Option<Vec<String>>,
    pub require_cn: Option<bool>,
    pub server_flag: Option<bool>,
    pub signature_bits: Option<i32>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<i32>,
    pub use_csr_common_name: Option<bool>,
    pub use_csr_sans: Option<bool>,
    pub use_pss: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RootGenerateRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub issuer_name: Option<String>,
    pub key_bits: Option<i32>,
    pub key_name: Option<String>,
    pub key_ref: Option<String>,
    pub key_type: Option<String>,
    pub locality: Option<Vec<String>>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
    pub max_path_length: Option<i32>,
    pub not_after: Option<String>,
    pub not_before_duration: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub permitted_dns_domains: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub use_pss: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RootReplaceRequest {
    pub default: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RootRotateRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub issuer_name: Option<String>,
    pub key_bits: Option<i32>,
    pub key_name: Option<String>,
    pub key_ref: Option<String>,
    pub key_type: Option<String>,
    pub locality: Option<Vec<String>>,
    pub managed_key_id: Option<String>,
    pub managed_key_name: Option<String>,
    pub max_path_length: Option<i32>,
    pub not_after: Option<String>,
    pub not_before_duration: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub permitted_dns_domains: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub use_pss: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RootSignIntermediateRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub csr: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub issuer_name: Option<String>,
    pub issuer_ref: Option<String>,
    pub locality: Option<Vec<String>>,
    pub max_path_length: Option<i32>,
    pub not_after: Option<String>,
    pub not_before_duration: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub permitted_dns_domains: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub skid: Option<String>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub use_csr_values: Option<bool>,
    pub use_pss: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct RootSignSelfIssuedRequest {
    pub certificate: Option<String>,
    pub issuer_ref: Option<String>,
    pub require_matching_certificate_algorithms: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct SignRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub csr: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub issuer_ref: Option<String>,
    pub not_after: Option<String>,
    pub other_sans: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub remove_roots_from_chain: Option<bool>,
    pub serial_number: Option<String>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct SignVerbatimRequest {
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub csr: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub ext_key_usage: Option<Vec<String>>,
    pub ext_key_usage_oids: Option<Vec<String>>,
    pub format: Option<String>,
    pub ip_sans: Option<Vec<String>>,
    pub issuer_ref: Option<String>,
    pub key_usage: Option<Vec<String>>,
    pub not_after: Option<String>,
    pub other_sans: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub remove_roots_from_chain: Option<bool>,
    pub serial_number: Option<String>,
    pub signature_bits: Option<i32>,
    pub ttl: Option<i32>,
    pub uri_sans: Option<Vec<String>>,
    pub use_pss: Option<bool>,
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct TidyRequest {
    pub issuer_safety_buffer: Option<i32>,
    pub pause_duration: Option<String>,
    pub revocation_queue_safety_buffer: Option<i32>,
    pub safety_buffer: Option<i32>,
    pub tidy_cert_store: Option<bool>,
    pub tidy_cross_cluster_revoked_certs: Option<bool>,
    pub tidy_expired_issuers: Option<bool>,
    pub tidy_move_legacy_ca_bundle: Option<bool>,
    pub tidy_revocation_list: Option<bool>,
    pub tidy_revocation_queue: Option<bool>,
    pub tidy_revoked_cert_issuer_associations: Option<bool>,
    pub tidy_revoked_certs: Option<bool>,
}