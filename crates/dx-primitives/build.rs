use dx_css_build::css_build;

fn main() {
    css_build();
}

fn _print_env() {
    for (k, v) in std::env::vars() {
        println!("cargo:warning=ENV: {k}='{v}'");
    }
}
