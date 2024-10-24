//! Requirement Tracing Tool

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unreachable_code)
)]

use std::fs;
use std::io::Write;
use std::path::PathBuf;

macro_rules! requirement_covered {
    ($id:ident) => {};
    ($id:ident,$title:literal) => {};
    ($id:ident,$title:literal) => {};
}

mod common;
mod controller;
mod errors;
mod formatters;
mod graph;
mod models;
mod parsers;
mod trace;
mod util;

use clap::Parser;

use self::models::Config;

/// A `StructOpt` example
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    quiet: bool,

    #[arg(short, long)]
    log_level: Option<String>,

    #[arg(short, long = "config", default_value = "requirements.toml")]
    config_file: PathBuf,

    #[arg()]
    jobs: Vec<String>,
}

fn logging_setup(opt: &Arguments) {
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

    if let Some(ref ll) = opt.log_level {
        builder.parse_filters(ll);
    }
    builder.init();
}

fn get_config(opt: &Arguments) -> Result<Config, Box<dyn std::error::Error>> {
    log::info!("using config file {}", opt.config_file.display());
    let config: Config = toml::from_slice(
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
    requirement_covered!(DSG_CONFIG_TOML, "Use a Single TOML File as Configuration");

    Ok(config)
}

fn run_cli_jobs(
    controller: &controller::Controller,
    opt: &Arguments,
) -> Result<bool, Box<dyn std::error::Error>> {
    let res = if opt.jobs.is_empty() {
        controller.run_default_jobs()
    } else {
        requirement_covered!(DSG_JOBS);
        controller.run_jobs_by_name(&opt.jobs)
    };
    requirement_covered!(DSG_CLI);

    res.map_err(|e: errors::Error| Box::new(e).into())
}

fn try_main() -> Result<bool, Box<dyn std::error::Error>> {
    let opt: Arguments = Arguments::parse();
    logging_setup(&opt);
    let config = get_config(&opt)?;
    let controller = controller::Controller::new(config)?;
    run_cli_jobs(&controller, &opt)
}

fn main_rc() -> i32 {
    let r = try_main();

    requirement_covered!(DSG_CLI_RETURN_CODE);

    match r {
        Err(e) => {
            log::error!("{}", e);
            eprintln!("Fatal Error: {e}");
            2
        }
        Ok(true) => 0,
        Ok(false) => 1,
    }
}

fn main() {
    std::process::exit(main_rc());
}
