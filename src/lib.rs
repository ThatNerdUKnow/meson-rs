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
//! extern crate meson;
//! use std::env;
//! use std::path::PathBuf;
//!
//! fn main() {
//!     let build_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");
//!     let build_path = build_path.to_str().unwrap();
//!
//!     println!("cargo:rustc-link-lib=squid");
//!     println!("cargo:rustc-link-search=native={}", build_path);
//!     meson::build("clib", build_path);
//! }
//! ```
//!
//! Cargo.toml:
//!
//! ```toml
//! # ...
//!
//! [build-dependencies]
//! meson = "1.0.0"
//! ```
//!
//! meson.build:
//!
//! ```text
//! project('squid', 'c')
//! shared_library('squid', 'squid.c')
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::{env, vec};

/// Runs meson and/or ninja to build a project.
pub fn build(project_dir: &str, build_dir: &str, options: Option<HashMap<&str, &str>>) {
    run_meson(project_dir, build_dir, options);
}

fn run_meson(lib: &str, dir: &str, options: Option<HashMap<&str, &str>>) {
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

        if let Some(options) = options {
            let options: Vec<String> = options
                .keys()
                .into_iter()
                .map(|key| format!("-D{}={}", key, options.get(key).unwrap()))
                .collect();

            for option in options {
                args.insert(3, option);
            }
        } 

        let args:Vec<&str> = args.iter().map(|s|&**s).collect();

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
