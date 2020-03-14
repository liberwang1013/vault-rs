use crate::{VaultClient, VaultSecret};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Default)]
pub struct CreateTokenRequest {
    pub id: Option<String>,
    pub policies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub meta: HashMap<String, String>,
    pub lease: Option<String>,
    pub ttl: Option<String>,
    pub explicit_max_ttl: Option<String>,
    pub period: Option<String>,
    pub no_parent: Option<bool>,
    pub no_default_policy: Option<bool>,
    pub display_name: String,
    pub num_uses: i32,
    pub renewable: Option<bool>,
    #[serde(rename = "type")]
    pub token_type: String,
    pub entity_alias: String,
}

#[derive(Serialize, Default)]
pub struct LookupTokenRequest {
    token: String,
}

#[derive(Serialize, Default)]
struct RenewTokenRequest {
    token: String,
    increment: Option<String>,
}

#[derive(Serialize, Default)]
struct RenewTokenSelfRequest {
    increment: Option<String>,
}

impl VaultClient {
    pub async fn create_token(
        &self,
        req: &CreateTokenRequest,
    ) -> crate::Result<VaultSecret<()>> {
        self.post("auth/token/create", req)
            .await?
            .parse::<VaultSecret<()>>()
            .await
    }

    pub async fn create_token_orphan(
        &self,
        req: &CreateTokenRequest,
    ) -> crate::Result<VaultSecret<()>> {
        self.post("auth/token/create-orphan", req)
            .await?
            .parse::<VaultSecret<()>>()
            .await
    }

    pub async fn create_token_with_role(
        &self,
        role: &str,
        req: &CreateTokenRequest,
    ) -> crate::Result<VaultSecret<()>> {
        self.post(&format!("auth/token/create/{}", role), req)
            .await?
            .parse::<VaultSecret<()>>()
            .await
    }

    pub async fn lookup_token(
        &self,
        req: LookupTokenRequest,
    ) -> crate::Result<VaultSecret<()>> {
        self.post("auth/token/lookup", req)
            .await?
            .parse::<VaultSecret<()>>()
            .await
    }

    pub async fn lookup_token_self(&self) -> crate::Result<VaultSecret<()>> {
        self.get("auth/token/lookup-self")
            .await?
            .parse::<VaultSecret<()>>()
            .await
    }

    pub async fn renew_token(
        &self,
        token: &str,
        increment: Option<&str>,
    ) -> crate::error::Result<VaultSecret<()>> {
        self.post(
            "auth/token/renew",
            &RenewTokenRequest {
                token: String::from(token),
                increment: increment.map(|i| String::from(i)),
            },
        )
        .await?
        .parse::<VaultSecret<()>>()
        .await
    }

    pub async fn renew_token_self(
        &self,
        increment: Option<&str>,
    ) -> crate::error::Result<VaultSecret<()>> {
        self.post(
            "auth/token/renew-self",
            &RenewTokenSelfRequest {
                increment: increment.map(|i| String::from(i)),
            },
        )
            .await?
            .parse::<VaultSecret<()>>()
            .await
    }
}
