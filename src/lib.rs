#[macro_use]
extern crate serde;
extern crate serde_json;

extern crate bytes;
extern crate reqwest;

pub mod auth;
pub mod system;
mod client;
pub mod error;
pub mod response;
pub mod secrets;
pub use crate::client::Client;
pub use crate::error::Result;
pub use crate::response::Secret;
