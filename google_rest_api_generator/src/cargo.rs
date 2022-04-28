const CARGO_TOML: &str = r#"
[package]
name = "{crate_name}"
version = "{crate_version}"
authors = ["Glenn Griffin <ggriffiniii@gmail.com"]
edition = "2018"
# for now, let's not even accidentally publish these
publish = false

[features]
default = ["rustls-tls"]

native-tls = ["reqwest/native-tls"]
rustls-tls = ["reqwest/rustls-tls"]

[dependencies]
chrono = { version = "0.4", features = ["serde"] }
futures = "0.3"
google_api_auth = { git = "https://github.com/B4dM4n/google-apis-rs-generator" }
google_field_selector = { git = "https://github.com/B4dM4n/google-apis-rs-generator" }
mime = "0.3"
percent-encoding = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
textnonce = "1"
"#;

const DEP_REQWEST: &str = r#"
reqwest = { version = "0.11", default-features = false, features = ["json"] }"#;

const DEP_REQWEST_STREAM: &str = r#"
reqwest = { version = "0.11", default-features = false, features = ["json", "stream"] }"#;

const DEP_TOKIO_UTIL: &str = r#"
tokio-util = { version = "0.7", features = ["compat", "io"] }"#;

const DEP_GOOGLE_BYTES: &str = r#"
google_api_bytes = { git = "https://github.com/B4dM4n/google-apis-rs-generator" }"#;

pub(crate) fn cargo_toml(
    crate_name: &str,
    api: &shared::Api,
    include_bytes_dep: bool,
    include_reqwest_stream: bool,
    include_tokio_util: bool,
) -> String {
    let mut doc = CARGO_TOML
        .trim()
        .replace("{crate_name}", crate_name)
        .replace(
            "{crate_version}",
            api.lib_crate_version
                .as_ref()
                .expect("available crate version"),
        );

    if include_reqwest_stream {
        doc.push_str(DEP_REQWEST_STREAM)
    } else {
        doc.push_str(DEP_REQWEST)
    }

    if include_tokio_util {
        doc.push_str(DEP_TOKIO_UTIL)
    }

    if include_bytes_dep {
        doc.push_str(DEP_GOOGLE_BYTES);
    }
    doc.push('\n');

    doc
}
