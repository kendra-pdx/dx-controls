mod bundle;
mod cmd;
mod error;
mod tailwind;

use derive_new::new;
pub use error::*;

use std::{env, path::PathBuf, str::FromStr};

use serde::Deserialize;

type Manifest = cargo_manifest::Manifest<Metadata, cargo_manifest::Value>;

#[macro_export]
macro_rules! debug {
    ($fmt:literal, $($args:expr),+) => {
        #[cfg(feature = "debug")]
        println!(concat!("cargo::warning=", $fmt), $($args),+);
    };
}

#[derive(Debug, new)]
pub struct CssBuilt {
    pub tailwind_css: Option<PathBuf>,
    pub bundle_css: Option<PathBuf>,
}

pub struct CssBuildConfig {
    out_dir: PathBuf,
    cargo_manifest_path: PathBuf,
}

impl Default for CssBuildConfig {
    fn default() -> Self {
        let out_dir = env::var("OUT_DIR").unwrap();
        let out_dir = PathBuf::from_str(&out_dir).unwrap(); // this is infallible

        let manifest_path = env::var("CARGO_MANIFEST_PATH").unwrap();
        let cargo_manifest_path = PathBuf::from_str(&manifest_path).unwrap();

        Self {
            out_dir,
            cargo_manifest_path,
        }
    }
}

pub fn css_build(tailwind: bool, bundle: bool) -> Result<CssBuilt, Error> {
    debug!("hello {}", "world");
    // let out_dir = env::var("OUT_DIR")?;
    // let out_dir = PathBuf::from_str(&out_dir).unwrap(); // this is infallible

    // let manifest_path = env::var("CARGO_MANIFEST_PATH")?;
    let config = CssBuildConfig::default();

    println!("cargo:rerun-if-changed=build.rs");

    let manifest =
        Manifest::from_path_with_metadata(config.cargo_manifest_path.to_string_lossy().to_string())
            .expect("Failed to read Cargo.toml");
    let css_metadata = CssMetadata::from(manifest);

    let tailwind_out = if tailwind {
        let tailwind_out = tailwind::tailwind(&css_metadata, &config.out_dir)?;
        if let Some(tailwind_out) = &tailwind_out {
            println!(
                "cargo::metadata=tailwind_css={}",
                tailwind_out.to_string_lossy()
            );
        }
        tailwind_out
    } else {
        None
    };

    let bundle_out = if bundle {
        let bundle_out = bundle::bundle(&css_metadata, tailwind_out.clone(), &config.out_dir)?;
        if let Some(bundle_out) = &bundle_out {
            println!(
                "cargo::metadata=bundle_css={}",
                bundle_out.to_string_lossy()
            );
        }
        bundle_out
    } else {
        None
    };

    let built = CssBuilt::new(tailwind_out, bundle_out);
    Ok(built)
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
