use dx_css_build::css_build;

fn main() {
    let _ = css_build(true, true)
        .inspect_err(|e| println!("cargo::error=CSS build failed: {e}"))
        .inspect(|built| println!("cargo::warning=Built TailwindCSS: {built:?}",));
}

fn _print_env() {
    for (k, v) in std::env::vars() {
        println!("cargo:warning=ENV: {k}='{v}'");
    }
}
