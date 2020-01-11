#[macro_use] extern crate log;
use vault::secrets::totp::{TotpKey, TotpKeyRole, TotpKeyRoleGenerator};
fn main() {
    env_logger::init();
    let key = TotpKey::default();
    info!("{}", serde_json::to_string(&key).unwrap());

    let generate = TotpKey {
        generate: true,
        role: TotpKeyRole::Generator(TotpKeyRoleGenerator::default()),
        ..TotpKey::default()
    };
    info!("{}", serde_json::to_string(&generate).unwrap());
}
