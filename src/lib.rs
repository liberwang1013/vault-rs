#[macro_use] extern crate error_chain;

#[macro_use] extern crate serde;
//#[macro_use] extern crate log;
extern crate serde_json;

extern crate reqwest;

pub mod error;
mod client;
pub mod engines;

pub use crate::client::VaultClient;
//pub use crate::engines::aws::RoleData;
//pub use crate::engines::kv_v2::KVData;

#[cfg(test)]
mod tests {
    use crate::client::VaultClient;

    #[test]
    fn init() {
        env_logger::init();
        assert_eq!(1, 1);
    }


    #[test]
    fn role_data() {
        assert_eq!(1, 1);
    }
}
