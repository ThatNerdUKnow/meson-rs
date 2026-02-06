//! ## Build Example
//!
//! ```text
//! .
//! ├── build.rs
//! ├── Cargo.toml
//! ├── clib
//! │   ├── meson.build
//! │   ├── squid.h
//! │   └── squid.c
//! └── src
//!     └── lib.rs
//! ```
//!
//! build.rs:
//!
//! ```
//! extern crate meson_next as meson;
//! use std::env;
//! use std::path::PathBuf;
//! use std::collections::HashMap;
//!
//! fn main() {
//!     let build_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");
//!     let build_path = build_path.to_str().unwrap();
//!
//!     let mut options = HashMap::new();
//!     options.insert("key", "value");
//!
//!     let config = meson::Config::new().options(options);
//!
//!     println!("cargo:rustc-link-lib=squid");
//!     println!("cargo:rustc-link-search=native={}", build_path);
//!     meson::build("clib", build_path, config);
//! }
//! ```
//!
//! Cargo.toml:
//!
//! ```toml
//! # ...
//!
//! [build-dependencies]
//! meson-next = "1"
//! ```
//!
//! meson.build:
//!
//! ```text
//! project('squid', 'c')
//! shared_library('squid', 'squid.c')
//! ```

use std::path::PathBuf;
use std::process::Command;
use std::{env, vec};

pub use config::Config;
pub mod config;

/// Runs meson and/or ninja to build a project.
pub fn build(project_dir: &str, build_dir: &str, config: Config) {
    run_meson(project_dir, build_dir, config);
}

fn run_meson(lib: &str, dir: &str, config: Config) {
    if !is_configured(dir) {
        let profile: &str = match env::var("PROFILE").unwrap().as_str() {
            "release" => "release",
            "debug" => "debug",
            _ => unreachable!(),
        };

        let mut args: Vec<String> = vec!["setup", "--buildtype", profile, dir]
            .iter()
            .map(|a| a.to_string())
            .collect();

        // Apply build options
        if let Some(options) = config.options {
            let options: Vec<String> = options
                .keys()
                .into_iter()
                .map(|key| format!("-D{}={}", key, options.get(key).unwrap()))
                .collect();

            for option in options {
                args.insert(3, option);
            }
        }

        // Apply native file
        if let Some(native_file) = config.native_file {
            args.insert(
                3,
                native_file.into_os_string().to_str().unwrap().to_string(),
            );
            args.insert(3, "--native-file".to_string())
        }

        // Apply cross file
        if let Some(cross_file) = config.cross_file {
            args.insert(3, cross_file.into_os_string().to_str().unwrap().to_string());
            args.insert(3, "--cross-file".to_string())
        }

        // convert owned strings into string slices for run_command
        let args: Vec<&str> = args.iter().map(|s| &**s).collect();

        run_command(lib, "meson", &args)
    }
    run_command(dir, "ninja", &[]);
}

fn run_command(dir: &str, name: &str, args: &[&str]) {
    let mut cmd = Command::new(name);
    cmd.current_dir(dir);
    if args.len() > 0 {
        cmd.args(args);
    }
    let status = cmd.status().expect(&format!("cannot run command {name}"));
    assert!(status.success());
}

fn is_configured(dir: &str) -> bool {
    let path = PathBuf::from(dir).join("build.ninja");
    return path.as_path().exists();
}
