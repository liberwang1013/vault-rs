use crate::{VaultClient, VaultData};
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
    pub entity_alias: String
}

impl VaultClient {
    pub async fn create_token(
        &self,
        req: &CreateTokenRequest,
    ) -> crate::Result<VaultData<()>> {
        self.post("auth/token/create", req)
            .await?
            .parse::<VaultData<()>>()
            .await
    }
}
