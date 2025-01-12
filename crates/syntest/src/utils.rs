use proc_macro2::TokenStream;
use quote::quote;
use std::{fs, process};

fn on_error(file_path: &str) {
    let _ = fs::remove_file(file_path);
}

pub struct TestOutput {
    output: process::Output,
}

impl TestOutput {
    pub fn stdout(&self) -> String {
        String::from_utf8(self.output.stdout.clone()).unwrap()
    }

    pub fn stderr(&self) -> String {
        String::from_utf8(self.output.stderr.clone()).unwrap()
    }

    pub fn output(&self) -> &process::Output {
        &self.output
    }
}

pub fn create_bin_and_run(code: &TokenStream) -> TestOutput {
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

    TestOutput { output }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bin_and_run() {
        let code = quote! {
            println!("Hello, world!");
        };

        let result = create_bin_and_run(&code);

        assert_eq!(result.stdout(), "Hello, world!\n");
    }
}
