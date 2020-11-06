use crate::{Client, Secret};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SysPolicy {
    pub name: Option<String>,
    pub policy: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SysPolicies {
    pub keys: Vec<String>,
}

impl Client {
    pub async fn list_policies(&self) -> crate::Result<Secret<SysPolicies>> {
        self.list("sys/policies/acl")
            .await?
            .parse::<Secret<SysPolicies>>()
            .await
    }

    pub async fn get_policy(&self, name: &str) -> crate::Result<Secret<SysPolicy>> {
        self.get(&format!("sys/policies/acl/{}", name))
            .await?
            .parse::<Secret<SysPolicy>>()
            .await
    }

    pub async fn put_policy(&self, name: &str, policy: &str) -> crate::Result<()> {
        let req = SysPolicy {
            name: None,
            policy: String::from(policy),
        };
        self.put(&format!("sys/policies/acl/{}", name), req)
            .await
            .and_then(|_| Ok(()))
    }

    pub async fn delete_policy(&self, name: &str) -> crate::Result<()> {
        self.delete(&format!("sys/policies/acl/{}", name))
            .await
            .and_then(|_| Ok(()))
    }
}
