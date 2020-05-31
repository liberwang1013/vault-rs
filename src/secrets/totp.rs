use crate::{Client, Secret};
const DEFAULT_PATH_TOTP: &str = "totp";

#[derive(Deserialize, Serialize, Debug)]
pub struct TotpKey {
    pub generate: bool,
    pub period: i32,
    pub algorithm: String,
    pub digits: i32,
    #[serde(flatten)]
    pub role: TotpKeyRole,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TotpKeyRole {
    Provider(TotpKeyRoleProvider),
    Generator(TotpKeyRoleGenerator),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotpKeyRoleProvider {
    pub url: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotpKeyRoleGenerator {
    pub exporterd: bool,
    pub key_size: i32,
    pub issuer: String,
    pub account_name: String,
    pub skew: i32,
    pub qr_size: i32,
}

impl Default for TotpKeyRoleProvider {
    fn default() -> Self {
        TotpKeyRoleProvider {
            url: String::from(""),
            key: String::from(""),
        }
    }
}

impl Default for TotpKeyRoleGenerator {
    fn default() -> Self {
        TotpKeyRoleGenerator {
            exporterd: true,
            key_size: 20,
            issuer: String::from(""),
            account_name: String::from(""),
            skew: 1,
            qr_size: 200,
        }
    }
}

impl Default for TotpKey {
    fn default() -> Self {
        TotpKey {
            generate: false,
            period: 30,
            algorithm: String::from("SHA1"),
            digits: 6,
            role: TotpKeyRole::Provider(TotpKeyRoleProvider::default()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TotpCode {
    pub code: String,
}

#[derive(Deserialize)]
pub struct TotpVerifyResult {
    pub valid: bool,
}

impl Client {
    pub async fn create_totp_key(
        &self,
        mount: Option<&str>,
        key: &str,
        data: TotpKey,
    ) -> crate::Result<()> {
        self.post(
            &format!("{}/keys/{}", mount.unwrap_or(DEFAULT_PATH_TOTP), key),
            data,
        )
        .await
        .and_then(|_| Ok(()))
    }

    pub async fn read_totp_key(
        &self,
        mount: Option<&str>,
        key: &str,
    ) -> crate::Result<Secret<TotpKey>> {
        self.get(&format!(
            "{}/keys/{}",
            mount.unwrap_or(DEFAULT_PATH_TOTP),
            key
        ))
        .await?
        .parse::<Secret<TotpKey>>()
        .await
    }

    pub async fn generate_totp_code(
        &self,
        mount: Option<&str>,
        key: &str,
    ) -> crate::Result<Secret<TotpCode>> {
        self.get(&format!(
            "{}/code/{}",
            mount.unwrap_or(DEFAULT_PATH_TOTP),
            key
        ))
        .await?
        .parse::<Secret<TotpCode>>()
        .await
    }

    pub async fn validate_totp_code(
        &self,
        mount: Option<&str>,
        key: &str,
        code: TotpCode,
    ) -> crate::Result<Secret<TotpVerifyResult>> {
        self.post(
            &format!("{}/code/{}", mount.unwrap_or(DEFAULT_PATH_TOTP), key),
            code,
        )
        .await?
        .parse::<Secret<TotpVerifyResult>>()
        .await
    }
}
