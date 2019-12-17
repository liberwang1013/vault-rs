use crate::client::VaultClient;
use crate::engines::ResponseMetadata;

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

/*
impl crate::response::FromVaultResponse for AwsCredential {
    fn from_vault_response(rsp: &crate::response::VaultResponse) -> reqwest::Result<Self> {
        Ok()
        AwsCredential{
            access_key: String::from("ak"),
            secret_key: String::from("sk"),
            security_token: String::from("st")
        }
    }
}
 */

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
    ) -> reqwest::Result<GetRootConfigResponse> {
        self.http_client
            .get(&format!(
                "{}/{}/config/root",
                self.endpoint,
                mount.unwrap_or(DEFAULT_PATH_AWS)
            ))
            .send()
            .await?
            .json::<GetRootConfigResponse>()
            .await
    }

    pub async fn put_aws_root_config(
        &self,
        mount: Option<&str>,
        config: AwsRootConfig,
    ) -> reqwest::Result<reqwest::StatusCode> {
        Ok(self
            .http_client
            .post(&format!(
                "{}/{}/config/root",
                self.endpoint,
                mount.unwrap_or(DEFAULT_PATH_AWS)
            ))
            .json(&config)
            .send()
            .await?
            .status())
    }

    pub async fn generate_sts_credentials(
        &self,
        mount: Option<&str>,
        role: &str,
    ) -> reqwest::Result<GenerateStsCredentialResponse> {
        self.http_client
            .get(&format!(
                "{}/{}/sts/{}",
                self.endpoint,
                mount.unwrap_or(DEFAULT_PATH_AWS),
                role
            ))
            .send()
            .await?
            .json::<GenerateStsCredentialResponse>()
            .await
    }
}
