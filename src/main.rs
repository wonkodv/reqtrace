#![allow(unused_imports)] // TODO
#![allow(dead_code)] // TODO

use std::{convert::TryInto, fmt, fs::File};

mod common;
mod controller;
mod formatters;
mod parsers;
mod trace;
mod stringvault;
mod genericvalue;

struct StringError(String);
impl<T: fmt::Debug> From<T> for StringError {
    fn from(e: T) -> Self {
        Self(format!("{:?}", e))
    }
}

fn try_main() -> Result<(), StringError> {

    let f = File::open("requirements.json")?;

    let config: controller::Config = serde_json::from_reader(f)?;

    let mut c = controller::Controller::new(&config);
    c.load()?;

    let job = c.find_job("tags").unwrap();
    c.run(&job)?;
  //  let job = c.find_job("tmx").unwrap();
  //  c.run(&job)?;
    if c.success() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn main() {
    let r = try_main();
    if let Err(e) = r {
        eprintln!("{}", e.0);
        std::process::exit(1);
    }
}
