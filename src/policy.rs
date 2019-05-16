use reqwest::{Error, Method, Url};

use crate::client::VaultClient;
use crate::error::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct WritePolicyRequest {
    pub policy: String
}

#[derive(Deserialize)]
struct ListPolicyResponse {
    pub policies: Vec<String>
}

#[derive(Deserialize)]
struct ReadPolicyResponse {
    pub name: String,
    pub rules: String
}

impl VaultClient {

    fn policy_uri(&self, name: &str) -> Result<Url> {
        Ok(Url::parse(&format!("{}/v1/sys/policy/{}", self.endpoint, name))?)
    }

    fn get_list_method() -> Result<Method> {
        let list = Method::from_bytes("LIST".as_bytes())?;
        Ok(list)
    }

    pub fn list_policy(&self) -> Result<Vec<String>> {
        let rsp: ListPolicyResponse = self.http_client.request(VaultClient::get_list_method()?, &format!("{}/v1/sys/policy", self.endpoint))
            .send()?
            .error_for_status()?
            .json()?;
        Ok(rsp.policies)
    }

    pub fn read_policy(&self, name: &str) -> Result<String> {
        let rsp: ReadPolicyResponse = self.http_client.get(self.policy_uri(name)?)
            .send()?
            .error_for_status()?
            .json()?;

        Ok(rsp.rules)
    }

    pub fn write_policy(&self, policy: &str, content: &str) -> Result<()> {
        let req = WritePolicyRequest{
            policy: content.to_string()
        };

        let mut sys_result = self.http_client
            .put(self.policy_uri(policy)?)
            .json(&req)
            .send()?
            .error_for_status()?;
        Ok(())

    }

    pub fn delete_policy(&self, policy: &str) -> Result<()> {
        let mut rsp = self.http_client.delete(self.policy_uri(policy)?)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}