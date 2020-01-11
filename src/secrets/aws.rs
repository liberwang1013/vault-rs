use crate::client::VaultClient;
use crate::response::VaultData;
use std::env;

const DEFAULT_PATH_AWS: &str = "aws";

#[derive(Deserialize, Serialize, Default)]
pub struct AwsRootConfig {
    #[serde(default = "default_max_retries")]
    max_retries: i32,
    access_key: String,
    secret_key: String,
    #[serde(default = "default_region")]
    region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sts_endpoint: Option<String>,
}

fn default_max_retries () -> i32 {
    -1
}

fn default_region() -> Option<String> {
    if let Ok(region) = env::var("AWS_REGION") {
        return Some(region);
    }

    if let Ok(region) = env::var("AWS_DEFAULT_REGION") {
        return Some(region);
    }

    Some(String::from("us-east-1"))
}

#[derive(Deserialize)]
pub struct AwsCredential {
    pub access_key: String,
    pub secret_key: String,
    pub security_token: String,
}

#[derive(Deserialize)]
pub struct AwsRole {
    pub policy_document: String,
    pub policy_arns: Vec<String>,
    pub credential_types: Vec<String>,
    pub role_arns: Vec<String>,
}

impl VaultClient {
    pub async fn get_aws_root_config(
        &self,
        mount: Option<&str>,
    ) -> crate::Result<VaultData<AwsRootConfig>> {
        self.get(&format!(
            "{}/config/root",
            mount.unwrap_or(DEFAULT_PATH_AWS)
        ))
        .await?
        .parse::<VaultData<AwsRootConfig>>()
        .await
    }

    pub async fn put_aws_root_config(
        &self,
        mount: Option<&str>,
        config: AwsRootConfig,
    ) -> crate::Result<()> {
        self.post(
            &format!("{}/config/root", mount.unwrap_or(DEFAULT_PATH_AWS)),
            config,
        )
        .await
            .and_then(|_| Ok(()))
    }

    pub async fn get_role(
        &self,
        mount: Option<&str>,
        role: &str,
    ) -> crate::Result<VaultData<AwsRole>> {
        self.get(&format!(
            "{}/roles/{}",
            mount.unwrap_or(DEFAULT_PATH_AWS),
            role
        ))
        .await?
        .parse::<VaultData<AwsRole>>()
        .await
    }

    pub async fn generate_sts_credentials(
        &self,
        mount: Option<&str>,
        role: &str,
    ) -> crate::Result<VaultData<AwsCredential>> {
        self.get(&format!(
            "{}/sts/{}",
            mount.unwrap_or(DEFAULT_PATH_AWS),
            role
        ))
        .await?
        .parse::<VaultData<AwsCredential>>()
        .await
    }
}
