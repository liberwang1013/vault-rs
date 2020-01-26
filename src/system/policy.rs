use crate::{VaultClient, VaultData};

#[derive(Deserialize)]
pub struct SystemPolicy {
    pub name: String,
    pub rules: String,
}

#[derive(Deserialize)]
pub struct SystemPolicyList {
    pub keys: Vec<String>,
    pub policies: Vec<String>,
}

impl VaultClient {
    pub async fn list_policy(&self) -> crate::Result<VaultData<SystemPolicyList>> {
        self.get("sys/policy")
            .await?
            .parse::<VaultData<SystemPolicyList>>()
            .await
    }

    pub async fn get_policy(
        &self,
        name: &str,
    ) -> crate::Result<VaultData<SystemPolicy>> {
        self.get(&format!("system/policy/{}", name))
            .await?
            .parse::<VaultData<SystemPolicy>>()
            .await
    }
}
