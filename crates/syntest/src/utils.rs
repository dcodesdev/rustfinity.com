use proc_macro2::TokenStream;
use quote::quote;
use std::{fs, process};

fn on_error(file_path: &str) {
    let _ = fs::remove_file(file_path);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bin_and_run() {
        let code = quote! {
            println!("Hello, world!");
        };

        let output = create_bin_and_run(code);

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout, "Hello, world!\n");
    }
}
