use crate::response::Response;
use serde::Serialize;
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

fn on_response(rsp: reqwest::Response) -> crate::error::Result<Response> {
    rsp.error_for_status()
        .and_then(|res| Ok(Response(res)))
        .map_err(|e| {
            let code = e.status().unwrap();
            crate::error::status_code(e, code)
        })
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
            None => env::var(VAULT_TOKEN).map_err(crate::error::builder)?,
        };
        Client::new(addr, token)
    }
}

impl Client {
    pub fn new(addr: String, token: String) -> crate::error::Result<Client> {
        // let addr = env::var(VAULT_ADDR).unwrap_or_else(|_| String::from(DEFAULT_VAULT_ADDR));
        // let token = env::var(VAULT_TOKEN).map_err(crate::error::builder)?;

        let mut default_header = reqwest::header::HeaderMap::new();
        default_header.insert(VAULT_TOKEN_HEADER, token.parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(default_header)
            .build()
            .map_err(crate::error::builder)?;

        Ok(Client {
            endpoint: format!("{}/v1", addr),
            http_client: client,
        })
    }

    pub(crate) async fn post<R: Serialize>(
        &self,
        key: &str,
        data: R,
    ) -> crate::error::Result<Response> {
        self.http_client
            .post(&format!("{}/{}", self.endpoint, key))
            .json(&data)
            .send()
            .await
            .map_err(crate::error::reqwest)
            .and_then(on_response)
    }

    pub(crate) async fn put<R: Serialize>(
        &self,
        key: &str,
        data: R,
    ) -> crate::error::Result<Response> {
        self.http_client
            .post(&format!("{}/{}", self.endpoint, key))
            .json(&data)
            .send()
            .await
            .map_err(crate::error::reqwest)
            .and_then(on_response)
    }

    pub(crate) async fn get(&self, key: &str) -> crate::error::Result<Response> {
        self.http_client
            .get(&format!("{}/{}", self.endpoint, key))
            .send()
            .await
            .map_err(crate::error::reqwest)
            .and_then(on_response)
    }

    pub(crate) async fn delete(&self, key: &str) -> crate::error::Result<Response> {
        self.http_client
            .delete(&format!("{}/{}", self.endpoint, key))
            .send()
            .await
            .map_err(crate::error::reqwest)
            .and_then(on_response)
    }

    pub(crate) async fn list(&self, key: &str) -> crate::error::Result<Response> {
        self.http_client
            .request(
                reqwest::Method::from_bytes("LIST".as_bytes()).unwrap(),
                &format!("{}/{}", self.endpoint, key),
            )
            .send()
            .await
            .map_err(crate::error::reqwest)
            .and_then(on_response)
    }

    pub(crate) async fn get_with_query<T: Serialize + ?Sized>(
        &self,
        key: &str,
        query: &T,
    ) -> crate::error::Result<Response> {
        self.http_client
            .get(&format!("{}/{}", self.endpoint, key))
            .query(query)
            .send()
            .await
            .map_err(crate::error::reqwest)
            .and_then(on_response)
    }
}
