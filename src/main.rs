//! Requirement Tracing Tool

#![allow(unused_imports, dead_code)] // TODO
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#![allow(clippy::needless_lifetimes, clippy::enum_variant_names)]

use std::{
    convert::TryInto,
    fmt,
    fs::{self, File},
};

mod common;
mod controller;
mod errors;
mod formatters;
mod genericvalue;
mod graph;
mod parsers;
mod pool;
mod trace;

fn try_main() -> Result<bool, Box<dyn std::error::Error>> {
    let config: controller::Config = toml::from_slice(
        fs::read("requirements.toml")
            .map_err(|e| format!("requirements.toml: {}", e))?
            .as_slice(),
    )
    .map_err(|e| {
        if let Some((line, col)) = e.line_col() {
            format!("requirements.toml:{}:{}: TOML Error {}", line + 1, col, e)
        } else {
            format!("requirements.toml:  TOML Error {}", e)
        }
    })?;
    let mut c = controller::Controller::new(&config);
    c.load()?;

    let job = c.find_job("tmx").unwrap();
    c.run(&job)?;
    let job = c.find_job("tags").unwrap();
    c.run(&job)?;

    Ok(c.success())
}

fn main() {
    let r = try_main();
    match r {
        Err(e) => {
            eprintln!("Fatal Error: {}", e);
            std::process::exit(2);
        }
        Ok(true) => std::process::exit(0),
        Ok(false) => std::process::exit(1),
    }
}
