use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ServerMapping {
    id: String,
    name: String,
    website: Option<String>,
    store: Option<String>,
    addresses: Vec<String>,
    primary_address: Option<String>,
    primary_color: Option<String>,
    secondary_color: Option<String>,
    game_types: Option<Vec<String>>,
    minecraft_versions: Option<Vec<String>>,
    primary_minecraft_version: Option<String>,
    primary_language: Option<String>,
    languages: Option<Vec<String>>,
    regions: Option<Vec<String>>,
    socials: Option<ServerSocials>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ServerSocials {
    twitter: Option<String>,
    discord: Option<String>,
    youtube: Option<String>,
    instagram: Option<String>,
    twitch: Option<String>,
    telegram: Option<String>,
    reddit: Option<String>,
    tiktok: Option<String>,
    facebook: Option<String>,
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("mappings.rs");

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let mappings_dir = Path::new(&manifest_dir).join("mappings").join("servers");

    let server_dirs = mappings_dir.read_dir().unwrap();

    let mut builder = phf_codegen::Map::new();
    let mut duplicate_detection = HashSet::new();

    for server_dir in server_dirs {
        let mapping = server_dir.unwrap().path().join("metadata.json");
        let mapping = fs::read_to_string(mapping).unwrap();
        let mut mapping: ServerMapping = serde_json::from_str(&mapping).unwrap();

        if let Some(primary_address) = mapping.primary_address.take() {
            mapping.addresses.push(primary_address);
        }

        for address in mapping.addresses {
            if duplicate_detection.contains(&address) {
                continue;
            }
            duplicate_detection.insert(address.clone());
            builder.entry(address, &format!("\"{}\"", mapping.name));
        }
    }

    fs::write(
        dest_path,
        format!(
            r#"
/// Automatically generated server mappings.
/// Sourced from https://github.com/LunarClient/ServerMappings
/// Edit build.rs, not this file.

static SERVER_MAPPINGS: phf::Map<&'static str, &'static str> = {};
"#,
            builder.build()
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=mappings");
    println!("cargo:rerun-if-changed=build.rs");
}
