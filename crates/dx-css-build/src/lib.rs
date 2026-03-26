mod bundle;
mod cmd;
mod tailwind;

use std::{env, path::PathBuf, str::FromStr};

use serde::Deserialize;

type Manifest = cargo_manifest::Manifest<Metadata, cargo_manifest::Value>;

pub fn css_build() -> Option<()> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let out_dir = PathBuf::from_str(&out_dir).expect("could not convert OUT_DIR to a Path");

    let manifest_path = env::var("CARGO_MANIFEST_PATH").expect("CARGO_MANIFEST_PATH is not set");

    println!("cargo:rerun-if-changed=build.rs");

    let manifest =
        Manifest::from_path_with_metadata(manifest_path).expect("Failed to read Cargo.toml");
    
    let css_metadata = CssMetadata::from(manifest);

    let tailwind_out = tailwind::tailwind(&css_metadata, &out_dir)?;

    println!("cargo::metadata=cs={:?}", out_dir);
    None
}

#[derive(Debug, Deserialize, Clone)]
struct Metadata {
    css: Option<CssMetadata>,
}

#[derive(Debug, Deserialize, Clone)]
struct CssMetadata {
    tailwind_input: Option<String>,
}

impl Default for CssMetadata {
    fn default() -> Self {
        Self {
            tailwind_input: Some("tailwind.css".into()),
        }
    }
}

impl From<Manifest> for CssMetadata {
    fn from(value: Manifest) -> Self {
        let css = value.package.and_then(|p| p.metadata).and_then(|m| m.css);
        css.unwrap_or_default()
    }
}
