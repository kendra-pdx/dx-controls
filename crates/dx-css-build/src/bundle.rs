use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use lightningcss::{
    bundler::{Bundler, FileProvider},
    printer::PrinterOptions,
    stylesheet::{MinifyOptions, ParserOptions},
};

use crate::{CssMetadata, Error, debug};

pub fn bundle(
    _manifest: &CssMetadata,
    tailwind_css: Option<PathBuf>,
    out_dir: &Path,
) -> Result<Option<PathBuf>, Error> {
    let mut sources = Vec::new();

    tailwind_css
        .into_iter()
        .chain(linked_css_sources())
        .for_each(|source| sources.push(source));

    if sources.is_empty() {
        return Ok(None);
    }

    let bundle_source = gen_bundle_source(out_dir, sources.into_iter())?;
    let bundle_css = gen_bundle(&bundle_source, &out_dir)?;

    debug!("bundle_css={}", bundle_css.to_string_lossy());

    Ok(Some(bundle_css))
}

fn linked_css_sources() -> impl Iterator<Item = PathBuf> {
    env::vars().into_iter().filter_map(|(key, value)| {
        if key.starts_with("DEP_") && key.ends_with("_BUNDLE_CSS") {
            debug!("Bundling dependency: {}", value);
            Some(PathBuf::from(value))
        } else {
            None
        }
    })
}

fn gen_bundle(source: &Path, out_dir: &Path) -> Result<PathBuf, Error> {
    let fs = FileProvider::new();
    let mut bundler = Bundler::new(&fs, None, ParserOptions::default());

    // bundle from the source
    let mut stylesheet = bundler
        .bundle(source)
        .map_err(|e| Error::message(e.to_string()))?;

    // minify
    stylesheet
        .minify(MinifyOptions::default())
        .map_err(|e| Error::message(e.to_string()))?;

    // generate css
    let printer_options = PrinterOptions {
        minify: true,
        ..PrinterOptions::default()
    };
    let css = stylesheet
        .to_css(printer_options)
        .map_err(|e| Error::message(e.to_string()))?;

    // write the generated css to a the output file
    let css_out_path = out_dir.join("bundle.css");
    let css_file = File::create(&css_out_path)?;
    let mut css_writer = BufWriter::new(css_file);
    css_writer.write_all(css.code.as_bytes())?;

    Ok(css_out_path)
}

fn gen_bundle_source(
    out_dir: &Path,
    sources: impl Iterator<Item = PathBuf>,
) -> Result<PathBuf, Error> {
    let bundle_source_path = out_dir.join("bundle_source.css");
    let bundle_source_file = File::create(&bundle_source_path)?;
    let mut bundle_source_writer = BufWriter::new(bundle_source_file);
    for source in sources {
        bundle_source_writer
            .write_fmt(format_args!(r#"@import "{}";"#, source.to_string_lossy()))?;
        bundle_source_writer.write(b"\n")?;
    }
    Ok(bundle_source_path)
}
