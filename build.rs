use openapi_struct_gen::generate;

fn build(spec: &str, generated: &str) {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let spec = format!("{}/{}", &dir, spec);
    let generated = format!("{}", generated);
    generate(
        spec,
        generated,
        Some(&["Clone", "Default", "Serialize", "Deserialize", "Builder"]),
        Some(&[
            ("serde", "Serialize"),
            ("serde", "Deserialize"),
            ("derive_builder", "Builder"),
        ]),
        Some(&[(r#"builder(setter(strip_option), default)"#, None)]),
        None,
    )
    .unwrap();
}

fn main() {
    build("spec/secrets/ad.json", "src/secrets/ad.rs");
    build("spec/secrets/alicloud.json", "src/secrets/alicloud.rs");
    build("spec/secrets/aws.json", "src/secrets/aws.rs");
    build("spec/secrets/azure.json", "src/secrets/azure.rs");
    build("spec/secrets/consul.json", "src/secrets/consul.rs");
    build("spec/secrets/database.json", "src/secrets/database.rs");
    build("spec/secrets/gcp.json", "src/secrets/gcp.rs");
    build("spec/secrets/gcpkms.json", "src/secrets/gcpkms.rs");
    build("spec/secrets/kv.json", "src/secrets/kv.rs");
    build("spec/secrets/kubernetes.json", "src/secrets/kubernetes.rs");
    build(
        "spec/secrets/mongodbatlas.json",
        "src/secrets/mongodbatlas.rs",
    );
    build("spec/secrets/nomad.json", "src/secrets/nomad.rs");
    build("spec/secrets/ldap.json", "src/secrets/ldap.rs");
    build("spec/secrets/pki.json", "src/secrets/pki.rs");
    build("spec/secrets/rabbitmq.json", "src/secrets/rabbitmq.rs");
    build("spec/secrets/ssh.json", "src/secrets/ssh.rs");
    build("spec/secrets/terraform.json", "src/secrets/terraform.rs");
    build("spec/secrets/totp.json", "src/secrets/totp.rs");
    build("spec/secrets/transit.json", "src/secrets/transit.rs");
    build("spec/secrets/kv-v2.json", "src/secrets/kv2.rs");

    build("spec/auth/alicloud.json", "src/auth/alicloud.rs");
    build("spec/auth/approle.json", "src/auth/approle.rs");
    build("spec/auth/aws.json", "src/auth/aws.rs");
    build("spec/auth/azure.json", "src/auth/azure.rs");
    build("spec/auth/cf.json", "src/auth/cf.rs");
    build("spec/auth/github.json", "src/auth/github.rs");
    build("spec/auth/gcp.json", "src/auth/gcp.rs");
    build("spec/auth/jwt.json", "src/auth/jwt.rs");
    build("spec/auth/kerberos.json", "src/auth/kerberos.rs");
    build("spec/auth/kubernetes.json", "src/auth/kubernetes.rs");
    build("spec/auth/ldap.json", "src/auth/ldap.rs");
    build("spec/auth/oci.json", "src/auth/oci.rs");
    build("spec/auth/okta.json", "src/auth/okta.rs");
    build("spec/auth/radius.json", "src/auth/radius.rs");
    build("spec/auth/cert.json", "src/auth/cert.rs");
    build("spec/auth/userpass.json", "src/auth/userpass.rs");
    build("spec/auth/token.json", "src/auth/token.rs");

    build("spec/sys.json", "src/sys/mod.rs");
}
