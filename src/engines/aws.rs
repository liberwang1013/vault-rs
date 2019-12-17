use crate::client::VaultClient;
use crate::engines::ResponseMetadata;

use crate::response::VaultData;

#[derive(Deserialize, Serialize)]
pub struct AwsRootConfig {
    max_retries: i32,
    access_key: String,
    secret_key: String,
    region: Option<String>,
}

#[derive(Deserialize)]
pub struct AwsCredential {
    pub access_key: String,
    pub secret_key: String,
    pub security_token: String,
}

#[derive(Deserialize)]
pub struct GetRootConfigResponse {
    pub data: AwsRootConfig,
}

#[derive(Deserialize)]
pub struct GenerateStsCredentialResponse {
    #[serde(flatten)]
    meta: ResponseMetadata,
    pub data: AwsCredential,
}

const DEFAULT_PATH_AWS: &str = "aws";
impl VaultClient {
    pub async fn get_aws_root_config(
        &self,
        mount: Option<&str>,
    ) -> crate::error::Result<VaultData<AwsRootConfig>> {
        self.get(&format!("{}/config/root", mount.unwrap_or(DEFAULT_PATH_AWS)))
            .await?
            .parse::<VaultData<AwsRootConfig>>()
            .await
    }

    pub async fn put_aws_root_config(
        &self,
        mount: Option<&str>,
        config: AwsRootConfig,
    ) -> crate::error::Result<reqwest::StatusCode> {
        self.post(&format!("{}/config/root", mount.unwrap_or(DEFAULT_PATH_AWS)), config)
            .await
            .and_then(|rsp| {Ok(rsp.status())})
    }

    pub async fn generate_sts_credentials(
        &self,
        mount: Option<&str>,
        role: &str,
    ) -> crate::error::Result<VaultData<AwsCredential>> {
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
