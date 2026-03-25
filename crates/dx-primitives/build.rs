use std::env;

fn main() {
    println!("cargo:warning=Hello from build.rs");
    print_env();
    unsafe {
        env::set_var("DX_PRIMITIVIES", "ran");
    }
}

fn print_env() {
    for (k, v) in std::env::vars() {
        println!("cargo:warning=ENV: {k}='{v}'");
    }
}
