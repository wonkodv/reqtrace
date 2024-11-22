use reqtrace::controller::JobSuccess;

mod utils;

#[test]
fn test() {
    let env = utils::test_environment(file!());
    let ctrl = env.controller();

    assert_eq!(
        ctrl.run_jobs_by_name(&["parse".to_owned()]),
        Ok(JobSuccess::Success)
    );

    env.assert_out_file_matches("requirements.json");

    println!("COVERED: DSG_CTRL_PARSE: Parse all Artefacts");
}
