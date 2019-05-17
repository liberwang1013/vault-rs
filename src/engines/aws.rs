
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

const AWS_DEFAULT_PATH: &str = "aws";

#[derive(Deserialize)]
struct ReadRoleResponse {
    data: RoleData
}

impl VaultClient {
    fn aws_url(&self, path: Option<&str>, role: &str) -> Result<Url> {
        Ok(Url::parse(&format!("{}/v1/{}/roles/{}", self.endpoint, path.unwrap_or(AWS_DEFAULT_PATH), role))?)
    }

    fn list_url(&self, path: Option<&str>) -> Result<Url> {
        Ok(Url::parse(&format!("{}/v1/{}/roles", path.unwrap_or(AWS_DEFAULT_PATH), self.endpoint))?)
    }

    pub fn create_aws_role(&self, path: Option<&str>, name: &str, data: &RoleData) -> Result<()> {
        let rsp = self.http_client
            .post(self.aws_url(path, name)?)
            .json(data)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn delete_aws_role(&self, path: Option<&str>, role: &str) -> Result<()> {
        let rsp = self.http_client.delete(self.aws_url(path, role)?)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn read_aws_role(&self, path: Option<&str>, role: &str) -> Result<RoleData> {
        let rsp: ReadRoleResponse = self.http_client.get(self.aws_url(path, role)?)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(rsp.data)
    }
}
