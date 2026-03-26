use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use which::which;

use crate::{CssMetadata, Manifest};

pub fn tailwind(manifest: &CssMetadata, out_dir: &Path) -> Option<PathBuf> {
    let tailwindcss_exe = tailwind_exe()?;
    let input = tailwind_input(manifest)?;
    None
}

/// gets the tailwind css input file.
fn tailwind_input(manifest: &CssMetadata) -> Option<PathBuf> {
    let input = manifest.tailwind_input?;
    PathBuf::from_str(&input).ok()
}

fn tailwind_exe() -> Option<PathBuf> {
    which("tailwindcss")
        .inspect_err(|_| {
            println!("cargo::error=tailwindcss not found");
        })
        .ok()
}
