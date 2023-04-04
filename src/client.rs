use crate::{Error, Result, Secret};
use http::Method;
use serde::{de::DeserializeOwned, Serialize};
use std::env;

const VAULT_TOKEN_HEADER: &str = "X-VAULT-TOKEN";
const VAULT_ADDR: &str = "VAULT_ADDR";
const VAULT_TOKEN: &str = "VAULT_TOKEN";
const DEFAULT_VAULT_ADDR: &str = "http://127.0.0.1:8200";

#[derive(Clone)]
pub struct Client {
    pub endpoint: String,
    pub http_client: reqwest::Client,
}

pub struct ClientBuilder {
    address: Option<String>,
    token: Option<String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            address: None,
            token: None,
        }
    }

    pub fn address(&mut self, address: String) -> &mut Self {
        self.address = Some(address);
        self
    }

    pub fn token(&mut self, token: String) -> &mut Self {
        self.token = Some(token);
        self
    }

    pub fn build(&self) -> crate::error::Result<Client> {
        let addr = match &self.address {
            Some(s) => s.to_owned(),
            None => env::var(VAULT_ADDR).unwrap_or_else(|_| String::from(DEFAULT_VAULT_ADDR)),
        };
        let token = match &self.token {
            Some(s) => s.to_owned(),
            None => env::var(VAULT_TOKEN).map_err(|_| Error::MissingToken)?,
        };
        Client::new(addr, token)
    }
}

impl Client {
    #[allow(dead_code)]
    pub fn new(addr: String, token: String) -> crate::error::Result<Client> {
        let mut default_header = reqwest::header::HeaderMap::new();
        default_header.insert(VAULT_TOKEN_HEADER, token.parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(default_header)
            .build()?;

        Ok(Client {
            endpoint: format!("{}/v1", addr),
            http_client: client,
        })
    }

    #[allow(dead_code)]
    pub async fn execute<R: DeserializeOwned>(&self, req: reqwest::Request) -> Result<Secret<R>> {
        self.http_client
            .execute(req)
            .await?
            .error_for_status()?
            .json::<Secret<R>>()
            .await
            .map_err(|e| crate::Error::Reqwest(e))
    }

    #[allow(dead_code)]
    pub async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        key: &str,
        data: T,
    ) -> Result<Secret<R>> {
        let req = self
            .http_client
            .request(Method::POST, format!("{}/{}", self.endpoint, key))
            .json(&data)
            .build()?;
        self.execute(req).await
    }

    #[allow(dead_code)]
    pub async fn put<T: Serialize, R: DeserializeOwned>(
        &self,
        key: &str,
        data: T,
    ) -> Result<Secret<R>> {
        let req = self
            .http_client
            .request(Method::PUT, format!("{}/{}", self.endpoint, key))
            .json(&data)
            .build()?;
        self.execute(req).await
    }

    #[allow(dead_code)]
    pub async fn get<R: DeserializeOwned>(
        &self,
        key: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<Secret<R>> {
        let mut builder = self
            .http_client
            .request(Method::GET, format!("{}/{}", self.endpoint, key));
        builder = if let Some(params) = params {
            builder.query(&params)
        } else {
            builder
        };

        let req = builder.build()?;
        self.execute(req).await
    }

    #[allow(dead_code)]
    pub async fn delete<R: DeserializeOwned>(&self, key: &str) -> Result<Secret<R>> {
        let req = self
            .http_client
            .request(Method::DELETE, format!("{}/{}", self.endpoint, key))
            .build()?;
        self.execute(req).await
    }

    #[allow(dead_code)]
    pub async fn list<R: DeserializeOwned>(&self, key: &str) -> Result<Secret<R>> {
        let req = self
            .http_client
            .request(
                reqwest::Method::from_bytes("LIST".as_bytes()).unwrap(),
                format!("{}/{}", self.endpoint, key),
            )
            .build()?;
        self.execute(req).await
    }
}
