use std::path::Path;

use reqtrace::controller::JobSuccess;

mod utils;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error + 'static>> {
    for test_file in glob::glob("tests/data/*/config.toml").expect("glob") {
        let test_file = test_file?;
        println!("Testing {test_file:?}");
        fun_name(&test_file).map_err(|e| format!("Error in {test_file:?}: {e:?}"))?
    }
    Ok(())
}

fn fun_name(test_file: &Path) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let test: toml::Value =
        toml::from_slice(std::fs::read(&test_file)?.as_slice()).expect("parse toml");
    let jobs: Vec<String> = test["test"]["jobs"].clone().try_into()?;
    let success: JobSuccess = test["test"]["success"].clone().try_into()?;
    let out_files: Vec<String> = test["test"]["out_files"].clone().try_into()?;
    let covers: Vec<String> = test["test"]["covers"].clone().try_into()?;
    let env = utils::TestEnv {
        test_dir: test_file.parent().ok_or("config has no parent")?.to_owned(),
    };

    let ctrl = env.controller();

    assert_eq!(ctrl.run_jobs_by_name(&jobs), Ok(success));

    for f in out_files {
        env.assert_out_file_matches(&f);
    }
    for cov in covers {
        println!("{:?} COVERS {}", test_file.parent().unwrap(), cov);
    }
    Ok(())
}
