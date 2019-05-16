use reqwest::{Certificate};

use std::env;
use std::fs::File;
use std::io::prelude::*;

use crate::error::*;

const VAULT_TOKEN_HEADER: &str = "X-VAULT-TOKEN";

const VAULT_ADDR: &str = "VAULT_ADDR";
const VAULT_TOKEN: &str = "VAULT_TOKEN";
const VAULT_CACERT: &str = "VAULT_CACERT";

const DEFAULT_VAULT_ADDR: &str = "http://127.0.0.1:8200";

pub struct VaultClient {
    pub endpoint:  String,
    pub token: String,
    pub http_client: reqwest::Client,
}

impl VaultClient {
    pub fn new() -> Result<VaultClient> {

        let addr = env::var(VAULT_ADDR).unwrap_or_else(|_e| String::from(DEFAULT_VAULT_ADDR));
        let token = env::var(VAULT_TOKEN)?;
        let cacert = env::var(VAULT_CACERT).unwrap_or_else(|_e| "".to_string());

        let mut default_header = reqwest::header::HeaderMap::new();
        default_header.insert(VAULT_TOKEN_HEADER, token.parse().unwrap());

        let mut client = reqwest::Client::builder().default_headers(default_header);

        if cacert.len() > 0 {

            let cert = VaultClient::get_certificate(&cacert)?;
            client = client.add_root_certificate(cert);
        }

        Ok(VaultClient{endpoint: addr, token: token, http_client: client.build().unwrap()})
    }

    fn get_certificate(cacert: &str) -> Result<Certificate> {
        let mut file = File::open(cacert)?;
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf)?;
        let cert = reqwest::Certificate::from_pem(&buf)?;
        Ok(cert)
    }


}

