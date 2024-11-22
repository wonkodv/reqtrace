use reqtrace::controller::JobSuccess;

mod utils;

#[test]
fn failing_job_sets_return_code() {
    let env = utils::test_environment(file!());
    let ctrl = env.controller();

    assert_eq!(
        ctrl.run_jobs_by_name(&["succeeds".to_owned()]),
        Ok(JobSuccess::Success)
    );
    assert_eq!(
        ctrl.run_jobs_by_name(&["fails".to_owned()]),
        Ok(JobSuccess::ErrorsDetected)
    );
    assert_eq!(
        ctrl.run_jobs_by_name(&["fails".to_owned(), "fails".to_owned()]),
        Ok(JobSuccess::ErrorsDetected)
    );
    assert_eq!(
        ctrl.run_jobs_by_name(&["succeeds".to_owned(), "fails".to_owned()]),
        Ok(JobSuccess::ErrorsDetected)
    );
    assert_eq!(
        ctrl.run_jobs_by_name(&["fails".to_owned(), "succeeds".to_owned()]),
        Ok(JobSuccess::ErrorsDetected)
    );
    println!("COVERED: DSG_CTRL_RETURN_CODE: Return Code Indicates if Job found Errors");
}
