use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Secret<T> {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: i32,
    pub data: T,
    pub warnings: Option<Vec<String>>,
    pub auth: Option<SecretAuth>,
    pub wrap_infos: Option<SecretWrapInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecretAuth {
    pub client_token: String,
    pub accessor: String,
    pub policies: Vec<String>,
    pub token_policies: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub orphan: bool,
    pub entity_id: String,

    pub lease_duration: i32,
    pub renewable: bool,

    pub mfa_requirement: Option<MFARequirement>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecretWrapInfo {
    pub token: String,
    pub accessor: String,
    pub ttl: i32,
    pub creation_time: String,
    pub creation_path: String,
    pub wrapped_accessor: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MFAMethodID {
    pub r#type: Option<String>,
    pub id: Option<String>,
    pub uses_passcode: Option<bool>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MFAConstraintAny {
    pub any: Option<Vec<Option<MFAMethodID>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MFARequirement {
    pub mfa_request_id: Option<String>,
    pub mfa_constraints: Option<HashMap<String, Option<MFAConstraintAny>>>,
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::Secret;
    #[test]
    fn parse_secret() {
        let data = r#"
            {
                "request_id": "737d1a46-49b8-4ce2-141b-9409b9fc0d1d",
                "lease_id": "",
                "renewable": false,
                "lease_duration": 2764800,
                "data": {
                    "data": {
                        "abc": "d"
                    }
                },
                "wrap_info": null,
                "warnings": null,
                "auth": null
            }
            "#;

        assert_eq!(true, serde_json::from_str::<Secret<Value>>(data).is_ok());
    }
}
