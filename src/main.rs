//! Requirement Tracing Tool

#![allow(unused_imports, dead_code)] // TODO
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#![allow(clippy::needless_lifetimes, clippy::enum_variant_names)]

use log::*;
use std::{
    convert::TryInto,
    fmt,
    fs::{self, File},
    path::PathBuf,
};

use std::io::Write;

mod common;
mod controller;
mod errors;
mod formatters;
mod genericvalue;
mod graph;
mod parsers;
mod trace;

use structopt::StructOpt;

/// A StructOpt example
#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    #[structopt(short = "l", long = "log-level")]
    log_level: Option<String>,

    #[structopt(short = "c", long = "config", default_value = "requirements.toml")]
    config_file: PathBuf,

    #[structopt()]
    jobs: Vec<String>,
}

fn try_main() -> Result<bool, Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    // Requires MAN_LOG_CONFIG
    let mut builder = env_logger::Builder::new();
    builder
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{}: [{}] {}",
                record.file().unwrap_or("<no file>"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .parse_env("REQTRACE_LOG");

    match opt.log_level {
        Some(ref ll) => {
            builder.parse_filters(ll);
        }
        _ => (),
    }

    builder.init();

    info!("using config file {}", opt.config_file.display());
    let config: controller::Config = toml::from_slice(
        fs::read(&opt.config_file)
            .map_err(|e| format!("{}: {}", &opt.config_file.display(), e))?
            .as_slice(),
    )
    .map_err(|e| {
        if let Some((line, col)) = e.line_col() {
            format!(
                "{}:{}:{}: TOML Error {}",
                &opt.config_file.display(),
                line + 1,
                col,
                e
            )
        } else {
            format!("{}:  TOML Error {}", &opt.config_file.display(), e)
        }
    })?;

    let c = controller::Controller::new(&config)?;

    let res = if opt.jobs.is_empty() {
        c.run_default_jobs()
    } else {
        c.run_jobs_by_name(&opt.jobs)
    };

    res.map_err(|e: errors::Error| Box::new(e).into())
}

fn main() {
    let r = try_main();
    match r {
        Err(e) => {
            error!("{}", e);
            eprintln!("Fatal Error: {}", e);
            std::process::exit(2);
        }
        Ok(true) => std::process::exit(0),
        Ok(false) => std::process::exit(1),
    }
}
