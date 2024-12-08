use std::path::Path;

use reqtrace::controller::JobSuccess;

mod utils;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error + 'static>> {
    utils::setup_logging();
    for test_file in glob::glob("tests/data/*/config.toml").expect("glob") {
        let test_file = test_file?;
        log::info!("Data Test with {test_file:?}");
        run_one_test(&test_file).map_err(|e| format!("{test_file:?}:{e:?}"))?
    }
    Ok(())
}

fn run_one_test(test_file: &Path) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let test: toml::Value = toml::from_slice(std::fs::read(&test_file)?.as_slice())?;
    let jobs: Vec<String> = test["test"]["jobs"].clone().try_into()?;
    let expected_success: JobSuccess = test["test"]["success"].clone().try_into()?;
    let out_files: Vec<String> = test["test"]["out_files"].clone().try_into()?;
    let covers: Vec<String> = test["test"]["covers"].clone().try_into()?;
    let env = utils::TestEnv {
        test_dir: test_file.parent().ok_or("config has no parent")?.to_owned(),
    };

    let ctrl = env.controller();

    log::info!("running jobs {:?}", &jobs);
    let success = ctrl.run_jobs_by_name(&jobs);

    for f in out_files {
        env.assert_out_file_matches(&f);
    }

    assert_eq!(success, Ok(expected_success));

    for cov in covers {
        println!("{:?} COVERS {}", test_file.parent().unwrap(), cov);
    }
    Ok(())
}
