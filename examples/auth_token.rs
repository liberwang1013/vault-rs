#[macro_use]
extern crate log;
use tokio::runtime::Runtime;
use vault::VaultClient;
use vault::auth::token;
use std::collections::HashMap;

fn main() {
    env_logger::init();
    info!("test");
    let mut rt = Runtime::new().unwrap();
    let client = VaultClient::new().unwrap();
    let mut data = HashMap::<String, String>::new();
    data.insert(String::from("key"), String::from("value"));
    let req = token::CreateTokenRequest::default();


    let create_fut = client.create_token(&req);
    let create_rsp = rt.block_on(create_fut);
    info!("create rsp is {:?}", &create_rsp);
    if let Ok(rsp) = create_rsp {
        info!("rsp : {}", serde_json::to_string(&rsp).unwrap());
    }
}
