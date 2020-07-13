#[macro_use]
extern crate log;

extern crate serde_json;
use serde::Serialize;
use vault::secrets::aws::*;
use tokio::runtime::Runtime;

fn main() {
    env_logger::init();
    let mut arns = Vec::<String>::new();
    arns.push("arn:aws:iam::aws:policy/AmazonEC2ReadOnlyAccess".to_string());
    arns.push("arn:aws:iam::aws:policy/IAMReadOnlyAccess".to_string());
    let mut rt = Runtime::new().unwrap();
    let client = vault::Client::new().unwrap();
    let role = client.list_roles(None );
    match rt.block_on(role) {
        Ok(rsp) => {
            info!("{:?}", rsp);
            let s = serde_json::to_string(&rsp).unwrap();
            info!("{}", s);
        }

        Err(e) => {
            warn!("{:?}", e);
        }
    }

    // let user = IamUser {
    //     aws_iam_policy: AwsIamPolicConfig::PolicyArns(PolicyArns{policy_arns: arns}),
    //     default_sts_ttl: 0,
    //     max_sts_ttl: 0,
    //     permissions_boundary_arn: "".to_string(),
    // };

    // let role = AwsRole::IamUser(user);

    // let put_fut = client.put_role(None, "my-role-test", role.clone());
    // match rt.block_on(put_fut) {
    //     Ok(rsp) => {
    //         info!("{:?}", rsp);
    //         let s = serde_json::to_string(&rsp).unwrap();
    //         info!("{}", s);
    //     }

    //     Err(e) => {
    //         warn!("{:?}", e);
    //     }
    // }

    // let s = serde_json::to_string(&role).unwrap();
    // info!("{}", s);
}
