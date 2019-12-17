use crate::error::*;
//use crate::error::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::env;

use futures_util::TryFutureExt;


const VAULT_TOKEN_HEADER: &str = "X-VAULT-TOKEN";
const VAULT_ADDR: &str = "VAULT_ADDR";
const VAULT_TOKEN: &str = "VAULT_TOKEN";

const DEFAULT_VAULT_ADDR: &str = "http://127.0.0.1:8200";

pub struct VaultClient {
    pub endpoint: String,
    pub http_client: reqwest::Client,
}

use crate::response::VaultResponse;

impl VaultClient {
    pub fn new() -> Result<VaultClient> {
        let addr =
            env::var(VAULT_ADDR).unwrap_or_else(|_| String::from(DEFAULT_VAULT_ADDR));
        let token = env::var(VAULT_TOKEN).map_err(crate::error::builder)?;

        let mut default_header = reqwest::header::HeaderMap::new();
        default_header.insert(VAULT_TOKEN_HEADER, token.parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(default_header)
            .build().map_err(crate::error::builder)?;

        Ok(VaultClient {
            endpoint: format!("{}/v1", addr),
            http_client: client,
        })
    }

    // pub async fn post<R, D>(&self, key: &str, data: R) -> reqwest::Result<D>
    // where
    //     R: Serialize,
    //     D: DeserializeOwned,
    // {
    //     self.http_client
    //         .post(&format!("{}/{}", self.endpoint, key))
    //         .json(&data)
    //         .send()
    //         .await?
    //         .json::<D>()
    //         .await
    // }

    pub(crate) async fn post<R: Serialize>(&self, key: &str, data: R) -> crate::error::Result<VaultResponse> {
        self.http_client
            .post(&format!("{}/{}", self.endpoint, key))
            .json(&data)
            .send()
            .await
            .and_then(|rsp| Ok(VaultResponse(rsp)))
            .map_err(crate::error::reqwest)
    }

    pub(crate) async fn get(&self, key:  &str) -> crate::error::Result<VaultResponse> {
      self.http_client
            .get(&format!("{}/{}", self.endpoint,  key))
            .send()
            .await
            .and_then(|rsp| Ok(VaultResponse(rsp)))
            .map_err(crate::error::reqwest)
    }

    pub(crate) async fn get_with_query<T: Serialize + ?Sized>(&self, key:  &str, query: &T) -> crate::error::Result<VaultResponse> {
        self.http_client
            .get(&format!("{}/{}", self.endpoint,  key))
            .query(query)
            .send()
            .await
            .and_then(|rsp| Ok(VaultResponse(rsp)))
            .map_err(crate::error::reqwest)
    }
}
