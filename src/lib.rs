#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde;
//#[macro_use] extern crate log;
extern crate serde_json;

extern crate reqwest;
extern crate bytes;

mod client;
pub mod engines;
pub mod error;
pub mod response;
pub use crate::client::VaultClient;
//pub use crate::engines::aws::RoleData;
//pub use crate::engines::kv_v2::KVData;

