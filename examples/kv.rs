#[macro_use] extern crate log;
use tokio::runtime::Runtime;
use vault::VaultClient;
//use vault::error::*;
use vault::engines::kv2::{PutKV2Request, Kv2Config};

use std::collections::HashMap;

fn main() {
    env_logger::init();
    info!("test");
    let rt = Runtime::new().unwrap();
    let client = VaultClient::new().unwrap();

    let mut data = HashMap::<String, String>::new();
    data.insert(String::from("key"), String::from("value"));
    let req = PutKV2Request {
        data: data,
        ..Default::default()
    };
    let create_fut = client.put_kv(Some("kv"), "secret", &req);
    let create_rsp = rt.block_on(create_fut);
    info!("create rsp is {:?}", create_rsp);

    let fut = client.get_kv(Some("kv"), "secret", None);
    let rsp = rt.block_on(fut);
    info!("rsp is {:?}", rsp);


    let config_data = Kv2Config {
        max_versions: Some(7),
        ..Default::default()
    };

    let put_config_fut = client.put_config(Some("kv"), &config_data);
    let put_config_rsp = rt.block_on(put_config_fut);
    info!("put config rsp is {:?}", put_config_rsp);

    let config_fut = client.get_config(Some("kv"));
    let config_rsp = rt.block_on(config_fut);
    info!("config rsp is {:?}", config_rsp);
}
