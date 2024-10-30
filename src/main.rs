//! Requirement Tracing Tool

use clap::Parser;
use reqtrace::controller::Controller;
use reqtrace::controller::JobSuccess;
use reqtrace::models::Config;
use std::fs;
use std::io::Write as _;
use std::path::PathBuf;

macro_rules! requirement_covered {
    ($id:ident: $title:literal) => {};
}

static CONFIG_VERSION: u32 = 0; // currently unstable

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
    requirement_covered!(DSG_CTRL_CONFIG: "Single Config File");
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
    requirement_covered!(FMT_CONFIG_TOML: "Use a Single TOML File as Configuration");

    if let Some(version) = config.version {
        if version > CONFIG_VERSION {
            return Err(format!(
                "{} has unknown version {}",
                opt.config_file.display(),
                version
            )
            .into());
        }
    }

    Ok(config)
}

fn run_cli_jobs(
    controller: &Controller,
    opt: &Arguments,
) -> Result<JobSuccess, Box<dyn std::error::Error>> {
    let res = if opt.jobs.is_empty() {
        controller.run_default_jobs()
    } else {
        requirement_covered!(DSG_JOBS: "Jobs encode requested behavior");
        controller.run_jobs_by_name(&opt.jobs)
    };
    requirement_covered!(DSG_CLI: "Command Line Interface");

    res.map_err(|e| Box::new(e).into())
}

fn try_main() -> Result<JobSuccess, Box<dyn std::error::Error>> {
    let opt: Arguments = Arguments::parse();
    logging_setup(&opt);
    let config = get_config(&opt)?;
    let controller = Controller::new(config);
    run_cli_jobs(&controller, &opt)
}

fn main_rc() -> i32 {
    let r = try_main();

    requirement_covered!(DSG_CLI_RETURN_CODE: "Set return Code to indicate success");

    match r {
        Err(e) => {
            log::error!("{}", e);
            eprintln!("{e}");
            2
        }
        Ok(JobSuccess::Success) => 0,
        Ok(JobSuccess::ErrorsDetected) => 1,
    }
}

fn main() {
    std::process::exit(main_rc());
}
