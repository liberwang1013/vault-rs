#[macro_use]
extern crate serde;
extern crate serde_json;

extern crate bytes;
extern crate reqwest;

mod client;
pub mod secrets;
pub mod error;
pub mod response;
pub use crate::client::VaultClient;
pub use crate::error::Result;
pub use crate::response::VaultData;
