use std::{env, path::PathBuf};

pub fn main() {
    // println!("cargo::rerun-if-changed=tailwind.css");
    // println!("cargo::rerun-if-changed=src/");
    if let Ok(css) = dx_css_build::css_build(true, true)
        && let Some(bundle_css) = css.bundle_css
    {
        let cargo_dir = env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .expect("CARGO_MANIFEST_DIR is not set.");
        let bundle_to = cargo_dir.join("assets").join("bundle.css");
        std::fs::copy(bundle_css, bundle_to).expect("could not copy bundle to assets");
    } else {
        println!("cargo::warning=CSS was not bundled");
    }
}

fn _print_env() {
    for (k, v) in std::env::vars() {
        println!("cargo:warning=ENV: {k}='{v}'");
    }
}
