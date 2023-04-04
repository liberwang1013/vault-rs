use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct CodeRequest {
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
pub struct KeysRequest {
    pub account_name: Option<String>,
    pub algorithm: Option<String>,
    pub digits: Option<i32>,
    pub exported: Option<bool>,
    pub generate: Option<bool>,
    pub issuer: Option<String>,
    pub key: Option<String>,
    pub key_size: Option<i32>,
    pub period: Option<i32>,
    pub qr_size: Option<i32>,
    pub skew: Option<i32>,
    pub url: Option<String>,
}