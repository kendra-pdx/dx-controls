use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    CssMetadata, Error,
    cmd::{Cmd, TailwindCssCmd},
};

pub fn tailwind(manifest: &CssMetadata, out_dir: &Path) -> Result<Option<PathBuf>, Error> {
    let input = match tailwind_input(manifest) {
        Some(input) => input,
        None => return Ok(None),
    };

    let output = out_dir.join("tailwind_out.css");
    let cmd = TailwindCssCmd::new(input, output);
    cmd.execute()?;
    Ok(Some(cmd.output))
}

/// gets the tailwind css input file.
fn tailwind_input(manifest: &CssMetadata) -> Option<PathBuf> {
    let input = manifest.tailwind_input.clone()?;
    PathBuf::from_str(&input).ok()
}
