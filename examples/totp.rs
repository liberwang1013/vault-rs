#[macro_use] extern crate log;
use vault::secrets::totp::{TotpKey, TotpKeyRole, TotpKeyRoleGenerator, TotpKeyRoleProvider};
use vault::VaultClient;
use tokio::runtime::Runtime;

fn main() {
    env_logger::init();
    //let key = TotpKey::default();
    let key = TotpKey {
        role: TotpKeyRole::Provider(TotpKeyRoleProvider {
            url: "otpauth://totp/Google:test@gmail.com?secret=Y64VEVMBTSXCYIWRSHRNDZW62MPGVU2G&issuer=Google".to_owned(),
            ..TotpKeyRoleProvider::default()
        }),
        ..TotpKey::default()
    };
    info!("{}", serde_json::to_string(&key).unwrap());

    let generate = TotpKey {
        generate: true,
        role: TotpKeyRole::Generator(TotpKeyRoleGenerator::default()),
        ..TotpKey::default()
    };
    info!("{}", serde_json::to_string(&generate).unwrap());


    let mut rt = Runtime::new().unwrap();
    let client = VaultClient::new().unwrap();
    let rsp_fut = client.create_totp_key(None, "test", key);
    let create_rsp = rt.block_on(rsp_fut).unwrap();
    info!("{:?}", create_rsp);

    let code_fut = client.generate_totp_code(None, "test");
    let code = rt.block_on(code_fut).unwrap();
    info!("{:?}", code);
}
