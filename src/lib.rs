#[macro_use]
extern crate serde;
extern crate serde_json;

extern crate bytes;
extern crate reqwest;

pub mod auth;
mod client;
pub mod error;
pub mod response;
pub mod secrets;
pub mod sys;
pub use crate::client::{Client, ClientBuilder};
pub use crate::error::Result;
pub use crate::response::Secret;

pub use reqwest::{StatusCode, Url};
