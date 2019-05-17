use std::collections::HashMap;
use reqwest::{Url};

use crate::error::*;
use crate::client::VaultClient;

#[derive(Deserialize, Serialize, Default)]
pub struct KVData {
    data: HashMap<String, String>
}

#[derive(Deserialize)]
struct ReadKVResponse {
    data: KVData
}

impl KVData {
    pub fn add_kv(&mut self, key: &str, value: &str) {
        self.data.insert(String::from(key), String::from(value));
    }
}

impl PartialEq for KVData {
    fn eq(&self, other: &Self) -> bool {
        return crate::engines::hashmap_eq(&self.data, &other.data);
    }
}

impl Eq for KVData {}



impl VaultClient {
    fn data_url(&self, path: Option<&str>, kv: &str) -> Result<Url> {
        Ok(Url::parse(&format!("{}/v1/{}/data/{}", self.endpoint, path.unwrap_or("secret"), kv))?)
    }


    pub fn write_kv(&self, path: Option<&str>, kv: &str, data: &KVData) -> Result<()> {
        let rsp = self.http_client.post(self.data_url(path, kv)?)
            .json(data)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn read_kv(&self, path: Option<&str>, kv: &str) -> Result<KVData> {
        let rsp: ReadKVResponse = self.http_client.get(self.data_url(path, kv)?)
            .send()?
            .error_for_status()?
            .json()?;

        Ok(rsp.data)
    }

    pub fn delete_latest_kv(&self, path: Option<&str>, kv: &str) -> Result<()> {
        let rsp = self.http_client.delete(self.data_url(path, kv)?)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
