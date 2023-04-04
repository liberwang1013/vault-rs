pub mod auth;
mod client;
mod error;
mod secret;
pub mod secrets;
pub mod sys;
pub use crate::{
    client::{Client, ClientBuilder},
    error::{Error, Result},
    secret::Secret,
};

pub use reqwest::{StatusCode, Url};
