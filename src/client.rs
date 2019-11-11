use std::env;
//use std::fs::File;
//use std::io::prelude::*;

use crate::error::*;

const VAULT_TOKEN_HEADER: &str = "X-VAULT-TOKEN";

const VAULT_ADDR: &str = "VAULT_ADDR";
const VAULT_TOKEN: &str = "VAULT_TOKEN";

const DEFAULT_VAULT_ADDR: &str = "http://127.0.0.1:8200";

pub struct VaultClient {
    pub endpoint:  String,
    pub http_client: reqwest::Client,
}

impl VaultClient {
    pub fn new() -> Result<VaultClient> {

        let addr = env::var(VAULT_ADDR).unwrap_or_else(|_| String::from(DEFAULT_VAULT_ADDR));
        let token = env::var(VAULT_TOKEN)?;

        let mut default_header = reqwest::header::HeaderMap::new();
        default_header.insert(VAULT_TOKEN_HEADER, token.parse().unwrap());

        let client = reqwest::Client::builder().default_headers(default_header).build()?;

        Ok(VaultClient{endpoint: addr, http_client: client})
    }
}
