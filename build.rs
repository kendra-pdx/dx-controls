pub fn main() {
    println!("cargo:warning=building dx-controls");
    // dx_css_build::css_build();
    print_env();
}

fn print_env() {
    for (k, v) in std::env::vars() {
        println!("cargo:warning=ENV: {k}='{v}'");
    }
}
