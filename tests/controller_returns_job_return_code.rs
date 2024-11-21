use reqtrace::controller::JobSuccess;

mod utils;

#[test]
fn failing_job_sets_return_code() {
    let ctrl = utils::controller_for_this_test(file!());

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
}
