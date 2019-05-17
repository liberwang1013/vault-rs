#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;

#[macro_use] extern crate serde;
extern crate serde_json;

extern crate reqwest;

mod error;
mod client;
mod policy;
mod engines;

pub use crate::client::VaultClient;
pub use crate::engines::aws::RoleData;
pub use crate::engines::kv_v2::KVData;

#[cfg(test)]
mod tests {
    use crate::client::VaultClient;
    const POLICY: &str = "path \"aws/sts/test\" { capabilities = [\"read\", \"list\"] }";

    #[test]
    fn init() {
        env_logger::init();
        assert_eq!(1, 1);
    }

    #[test]
    fn list_policy() {
        let client = VaultClient::new().unwrap();
        let rsp = client.list_policy()
            .unwrap_or_else(|e| {
                error!("list_policy error : {}", e);
                assert_eq!(1, 0);
                Vec::<String>::new()
            });
        let expect = vec!["default".to_string(), "root".to_string()];
        assert_eq!(rsp, expect);
    }

    #[test]
    fn write_policy() {
        let client = VaultClient::new().unwrap();
        let result = client.write_policy("test", POLICY)
            .unwrap_or_else(|e| {
                error!("write_policy error : {}", e);
                assert_eq!(1, 0);
                ()
            });
        assert_eq!(1, 1)
    }

    #[test]
    fn read_policy() {
        let client = VaultClient::new().unwrap();
        let rsp = client.read_policy("test")
            .unwrap_or_else(|e| {
                error!("read_policy error : {}", e);
                assert_eq!(1, 0);
                "empty".to_string()
            });

        assert_eq!(rsp, POLICY.to_string());
    }

    #[test]
    fn delete_policy() {
        let client = VaultClient::new().unwrap();
        let rsp = client.delete_policy("test")
            .unwrap_or_else(|e| {
                error!("delete_policy error : {}", e);
                assert_eq!(0, 1);
                ()
            });
        assert_eq!(1, 1);
    }

    #[test]
    fn role_data() {
        assert_eq!(1, 1);
    }
}
