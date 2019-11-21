pub mod aws;
pub mod kv2;

#[derive(Deserialize, Serialize, Default, Debug)]
struct ResponseMetadata {
    request_id: String,
    lease_id: String,
    renewable: bool,
    lease_duration: i32,
    wrap_info: Option<String>,
    warnings: Option<String>,
    auth: Option<String>,
}
