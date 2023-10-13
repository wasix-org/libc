// Taken from https://github.com/dtolnay/rustversion. We can't add
// dependencies to this crate since practically everything else
// already depends on it, which is why we're copying a trimmed down
// version of the rustversion crate into this one directly.

#![allow(
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::single_match_else
)]

use std::env;
use std::ffi::OsString;
use std::process::{self, Command};

use super::rustc_version_parser;

pub fn generate_rustc_version() {
    println!("cargo:rerun-if-changed=build/build.rs");

    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));

    let mut is_clippy_driver = false;
    let version = loop {
        let mut command = Command::new(&rustc);
        if is_clippy_driver {
            command.arg("--rustc");
        }
        command.arg("--version");

        let output = match command.output() {
            Ok(output) => output,
            Err(e) => {
                let rustc = rustc.to_string_lossy();
                eprintln!("Error: failed to run `{} --version`: {}", rustc, e);
                process::exit(1);
            }
        };

        let string = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(e) => {
                let rustc = rustc.to_string_lossy();
                eprintln!(
                    "Error: failed to parse output of `{} --version`: {}",
                    rustc, e,
                );
                process::exit(1);
            }
        };

        break match rustc_version_parser::parse(&string) {
            rustc_version_parser::ParseResult::Success(version) => version,
            rustc_version_parser::ParseResult::OopsClippy if !is_clippy_driver => {
                is_clippy_driver = true;
                continue;
            }
            rustc_version_parser::ParseResult::Unrecognized
            | rustc_version_parser::ParseResult::OopsClippy => {
                eprintln!(
                    "Error: unexpected output from `rustc --version`: {:?}\n\n\
                    Please file an issue in https://github.com/dtolnay/rustversion",
                    string
                );
                process::exit(1);
            }
        };
    };

    // We know rustc 1.70 requires __wasm_signal to be exported, and rustc 1.73
    // doesn't. We have no information about 1.71 and 1.72, but since there
    // probably will never be a wasix toolchain with those versions, this check
    // should be sufficient.
    if version.minor <= 70 {
        println!("cargo:rustc-cfg=feature=\"export-wasm-signal\"");
    }
}
