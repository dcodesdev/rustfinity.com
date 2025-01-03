use proc_macro2::TokenStream;
use quote::quote;
use std::{fs, process};

fn on_error(file_path: &str) {
    fs::remove_file(file_path).expect("Failed to remove file");
}

pub fn create_bin_and_run(code: TokenStream) -> process::Output {
    let bin = "src/main.rs";

    let rust_code = quote! {
        fn main() {
            #code
        }
    };

    fs::write(bin, rust_code.to_string()).expect("Failed to write to file");

    let output = process::Command::new("cargo")
        .arg("run")
        .output()
        .or_else(|e| {
            on_error(bin);
            Err(e)
        })
        .expect("Failed to compile");

    fs::remove_file(bin).unwrap();

    output
}
