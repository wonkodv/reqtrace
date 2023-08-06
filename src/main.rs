//! Requirement Tracing Tool

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#![allow(clippy::needless_lifetimes, clippy::enum_variant_names)]
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unreachable_code)
)]

use std::fs;
use std::io::Write;
use std::path::PathBuf;

mod common;
mod controller;
mod errors;
mod formatters;
mod graph;
mod parsers;
mod trace;
mod util;

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

fn logging_setup(opt: &Opt) -> Result<(), Box<dyn std::error::Error>> {
    // Covers MAN_LOG_CONFIG: Configure Logging
    let mut builder = env_logger::Builder::new();
    builder
        .filter_level(log::LevelFilter::Info)
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

    Ok(())
}

fn get_config(opt: &Opt) -> Result<controller::Config, Box<dyn std::error::Error>> {
    log::info!("using config file {}", opt.config_file.display());
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
    cov_mark::hit!(DSG_CONFIG_TOML /* Use a Single TOML File as Configuration*/);

    Ok(config)
}

fn run_cli_jobs(
    controller: &controller::Controller,
    opt: &Opt,
) -> Result<bool, Box<dyn std::error::Error>> {
    let res = if opt.jobs.is_empty() {
        controller.run_default_jobs()
    } else {
        cov_mark::hit!(DSG_JOBS);
        controller.run_jobs_by_name(&opt.jobs)
    };
    cov_mark::hit!(DSG_CLI);

    res.map_err(|e: errors::Error| Box::new(e).into())
}

fn try_main() -> Result<bool, Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();
    logging_setup(&opt)?;
    let config = get_config(&opt)?;
    let controller = controller::Controller::new(config)?;
    run_cli_jobs(&controller, &opt)
}

fn main_rc() -> i32 {
    let r = try_main();
    let rc = match r {
        Err(e) => {
            log::error!("{}", e);
            eprintln!("Fatal Error: {}", e);
            2
        }
        Ok(true) => 0,
        Ok(false) => 1,
    };

    cov_mark::hit!(DSG_RETURN_CODE);

    rc
}

fn main() {
    std::process::exit(main_rc());
}
