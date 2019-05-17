
use std::default::Default;
use reqwest::{Url};
use serde::{Serialize, Deserialize};


use crate::error::*;
use crate::client::VaultClient;

#[derive(Serialize, Deserialize, Default)]
pub struct RoleData {
    pub credential_type: String,

    pub policy_document: String,

    pub policy_arns: Option<Vec<String>>,

    pub role_arns: Option<Vec<String>>,

    pub max_sts_ttl: i32,

    pub default_sts_ttl: i32,
}

#[derive(Deserialize)]
struct ReadRoleResponse {
    data: RoleData
}

impl VaultClient {
    fn aws_url(&self, role: &str) -> Result<Url> {
        Ok(Url::parse(&format!("{}/v1/aws/roles/{}", self.endpoint, role))?)
    }

    fn list_url(&self) -> Result<Url> {
        Ok(Url::parse(&format!("{}/v1/aws/roles", self.endpoint))?)
    }

    pub fn create_assumed_role(&self, name: &str, data: &RoleData) -> Result<()> {
        let rsp = self.http_client
            .post(self.aws_url(name)?)
            .json(data)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn delete_aws_role(&self, role: &str) -> Result<()> {
        let rsp = self.http_client.delete(self.aws_url(role)?)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn read_aws_role(&self, role: &str) -> Result<RoleData> {
        let data: RoleData = self.http_client.get(self.aws_url(role)?)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(data)
    }
}
