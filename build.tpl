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
