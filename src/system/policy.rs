use crate::{Client, Secret};

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

impl Client {
    pub async fn list_policy(&self) -> crate::Result<Secret<SystemPolicyList>> {
        self.get("sys/policy")
            .await?
            .parse::<Secret<SystemPolicyList>>()
            .await
    }

    pub async fn get_policy(
        &self,
        name: &str,
    ) -> crate::Result<Secret<SystemPolicy>> {
        self.get(&format!("system/policy/{}", name))
            .await?
            .parse::<Secret<SystemPolicy>>()
            .await
    }
}
