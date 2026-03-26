use std::env;

use cargo_manifest::Manifest;

pub fn css_build() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=Hello from CSS Build!");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let manifest_path = env::var("CARGO_MANIFEST_PATH").expect("CARGO_MANIFEST_PATH is not set");

    println!("cargo::warning=loading manifest: {}", manifest_path);
    let manifest = Manifest::from_path(manifest_path).expect("Failed to read Cargo.toml");
    if let Some(dependencies) = manifest.dependencies {
        for (name, dependency) in dependencies.iter() {
            let dependency = dependency.detail();
            println!("cargo::warning=Dependency: {name} = {dependency:?}");
        }
    }
    // let package_name = manifest.package.unwrap().name.to_uppercase();
    println!("cargo::metadata=CSS={}", out_dir);
}
