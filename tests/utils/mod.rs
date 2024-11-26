#![allow(dead_code)]

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use reqtrace::controller::Controller;
use reqtrace::models::Config;

pub fn setup_logging() {
    env_logger::builder().is_test(true).try_init().unwrap();
}

pub struct TestEnv {
    pub test_dir: PathBuf,
}

pub fn test_environment(test_file: &str) -> TestEnv {
    let test_file = Path::new(test_file);
    assert_eq!(test_file.extension().map(|s| s.to_str()), Some(Some("rs")));
    let test_dir = test_file.with_extension("");
    assert!(test_dir.is_dir());
    TestEnv { test_dir }
}

impl TestEnv {
    pub fn controller(&self) -> Controller {
        let config_path = self.test_dir.join("config.toml");
        let config: Config = toml::from_slice(
            std::fs::read(&config_path)
                .expect(&format!("can read {config_path:?}"))
                .as_slice(),
        )
        .expect(&format!("can parse {}", config_path.display()));

        Controller::new(config, &self.test_dir)
    }

    pub fn assert_out_file_matches(&self, file_to_test: &str) {
        let produced_file = self.test_dir.join("out").join(file_to_test);
        let expected_file = self.test_dir.join("expected").join(file_to_test);

        log::info!("comparing {produced_file:?} with {expected_file:?}");

        let produced = std::fs::read(&produced_file).expect(&format!("can read {produced_file:?}"));
        let expected = std::fs::read(&expected_file).expect(&format!("can read {expected_file:?}"));

        if produced != expected {
            let out = Command::new("git")
                .arg("diff")
                .arg("--no-index")
                .arg(produced_file)
                .arg(expected_file)
                .output()
                .expect("git diff failed to start");
            panic!(
                "files do not match: {}",
                String::from_utf8_lossy(&out.stdout)
            );
        }
    }
}
