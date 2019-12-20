use crate::{VaultClient, VaultData};
const DEFAULT_PATH_TOTP: &str = "totp";

#[derive(Deserialize, Serialize)]
pub struct TotpKey {
    pub generate: bool,
    pub exporterd: bool,
    pub key_size: i32,
    pub url: String,
    pub key: String,
    pub issuer: String,
    pub account_name: String,
    pub period: i32,
    pub algorithm: String,
    pub digits: i32,
    pub skew: i32,
    pub qr_size: i32,
}

impl Default for TotpKey {
    fn default() -> Self {
        TotpKey {
            generate: false,
            exporterd: true,
            key_size: 20,
            url: String::from(""),
            key: String::from(""),
            issuer: String::from(""),
            account_name: String::from(""),
            period: 30,
            algorithm: String::from("SHA1"),
            digits: 6,
            skew: 1,
            qr_size: 200,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TotpCode {
    pub code: String,
}

#[derive(Deserialize)]
pub struct TotpVerifyResult {
    pub valid: bool,
}

impl VaultClient {
    pub async fn crate_totp_key(
        &self,
        mount: Option<&str>,
        key: &str,
        data: TotpKey,
    ) -> crate::Result<VaultData<TotpKey>> {
        self.post(
            &format!("{}/keys/{}", mount.unwrap_or(DEFAULT_PATH_TOTP), key),
            data,
        )
        .await?
        .parse::<VaultData<TotpKey>>()
        .await
    }

    pub async fn read_totp_key(
        &self,
        mount: Option<&str>,
        key: &str,
    ) -> crate::Result<VaultData<TotpKey>> {
        self.get(&format!(
            "{}/keys/{}",
            mount.unwrap_or(DEFAULT_PATH_TOTP),
            key
        ))
        .await?
        .parse::<VaultData<TotpKey>>()
        .await
    }

    pub async fn generate_totp_code(
        &self,
        mount: Option<&str>,
        key: &str,
    ) -> crate::Result<VaultData<TotpCode>> {
        self.get(&format!(
            "{}/code/{}",
            mount.unwrap_or(DEFAULT_PATH_TOTP),
            key
        ))
        .await?
        .parse::<VaultData<TotpCode>>()
        .await
    }

    pub async fn validate_totp_code(
        &self,
        mount: Option<&str>,
        key: &str,
        code: TotpCode,
    ) -> crate::Result<VaultData<TotpVerifyResult>> {
        self.post(
            &format!("{}/code/{}", mount.unwrap_or(DEFAULT_PATH_TOTP), key),
            code,
        )
        .await?
        .parse::<VaultData<TotpVerifyResult>>()
        .await
    }
}
