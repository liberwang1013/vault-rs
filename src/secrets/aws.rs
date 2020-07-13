use crate::client::Client;
use crate::response::Secret;
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

fn default_max_retries() -> i32 {
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

#[derive(Deserialize, Serialize)]
pub struct AwsLease {
    pub lease: String,
    pub lease_max: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "credential_type")]
pub enum AwsRole {
    #[serde(rename = "iam_user")]
    IamUser(IamUser),
    #[serde(rename = "assumed_role")]
    AssumedRole(AssumedRole),
    #[serde(rename = "federation_token")]
    FederationToken(FederationToken),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolicyArns {
    pub policy_arns: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolicyDocument {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub policy_document: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AwsIamPolicConfig {
    PolicyArns(PolicyArns),
    PolicyDocument(PolicyDocument),
}

fn default_user_path() -> String {
    "/".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssumedRole {
    pub role_arns: Vec<String>,
    #[serde(default = "default_user_path")]
    pub user_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IamUser {
    #[serde(flatten)]
    pub aws_iam_policy: AwsIamPolicConfig,
    pub default_sts_ttl: i32,
    pub max_sts_ttl: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub permissions_boundary_arn: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FederationToken {
    #[serde(flatten)]
    pub aws_iam_policy: AwsIamPolicConfig,
    pub default_sts_ttl: i32,
    pub max_sts_ttl: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AwsRoles {
    #[serde(rename = "keys")]
    pub roles: Vec<String>,
}

impl Client {
    pub async fn get_aws_root_config(
        &self,
        mount: Option<&str>,
    ) -> crate::Result<Secret<AwsRootConfig>> {
        self.get(&format!(
            "{}/config/root",
            mount.unwrap_or(DEFAULT_PATH_AWS)
        ))
        .await?
        .parse::<Secret<AwsRootConfig>>()
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

    pub async fn put_aws_lease(&self, mount: Option<&str>, lease: AwsLease) -> crate::Result<()> {
        self.post(
            &format!("{}/config/lease", mount.unwrap_or(DEFAULT_PATH_AWS)),
            lease,
        )
        .await
        .and_then(|_| Ok(()))
    }

    pub async fn get_aws_lease(&self, mount: Option<&str>) -> crate::Result<Secret<AwsLease>> {
        self.get(&format!(
            "{}/config/lease",
            mount.unwrap_or(DEFAULT_PATH_AWS)
        ))
        .await?
        .parse::<Secret<AwsLease>>()
        .await
    }

    pub async fn put_role(
        &self,
        mount: Option<&str>,
        name: &str,
        role: AwsRole,
    ) -> crate::Result<()> {
        self.post(
            &format!("{}/roles/{}", mount.unwrap_or(DEFAULT_PATH_AWS), name,),
            role,
        )
        .await
        .and_then(|_| Ok(()))
    }

    pub async fn get_role(
        &self,
        mount: Option<&str>,
        role: &str,
    ) -> crate::Result<Secret<AwsRole>> {
        self.get(&format!(
            "{}/roles/{}",
            mount.unwrap_or(DEFAULT_PATH_AWS),
            role
        ))
        .await?
        .parse::<Secret<AwsRole>>()
        .await
    }

    pub async fn list_roles(&self, mount: Option<&str>) -> crate::Result<Secret<AwsRoles>> {
        self.list(&format!("{}/roles", mount.unwrap_or(DEFAULT_PATH_AWS)))
            .await?
            .parse::<Secret<AwsRoles>>()
            .await
    }

    pub async fn delete_role(&self, mount: Option<&str>, name: &str) -> crate::Result<()> {
        self.delete(&format!(
            "{}/roles/{}",
            mount.unwrap_or(DEFAULT_PATH_AWS),
            name
        ))
        .await
        .and_then(|_| Ok(()))
    }

    pub async fn generate_sts_credentials(
        &self,
        mount: Option<&str>,
        role: &str,
    ) -> crate::Result<Secret<AwsCredential>> {
        self.get(&format!(
            "{}/sts/{}",
            mount.unwrap_or(DEFAULT_PATH_AWS),
            role
        ))
        .await?
        .parse::<Secret<AwsCredential>>()
        .await
    }

    pub async fn generate_credentials(
        &self,
        mount: Option<&str>,
        role: &str,
    ) -> crate::Result<Secret<AwsCredential>> {
        self.get(&format!(
            "{}/creds/{}",
            mount.unwrap_or(DEFAULT_PATH_AWS),
            role
        ))
        .await?
        .parse::<Secret<AwsCredential>>()
        .await
    }
}
