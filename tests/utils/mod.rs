use std::path::Path;
use std::path::PathBuf;

use reqtrace::controller::Controller;
use reqtrace::models::Config;

pub fn test_dir_from_test_file(test_file: &str) -> PathBuf {
    let test_file = Path::new(test_file);
    assert_eq!(test_file.extension().map(|s| s.to_str()), Some(Some("rs")));
    let dir = test_file.with_extension("");
    assert!(dir.is_dir());
    dir
}

pub fn controller_for_this_test(test_file: &str) -> Controller {
    let test_dir = test_dir_from_test_file(test_file);
    let config_path = test_dir.join("config.toml");
    let config: Config = toml::from_slice(
        std::fs::read(&config_path)
            .expect(&format!("can read {test_dir:?}/config.toml"))
            .as_slice(),
    )
    .expect(&format!("can parse {}", config_path.display()));

    Controller::new(config)
}
