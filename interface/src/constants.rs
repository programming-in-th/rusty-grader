use grader::submission::SubmissionStatus;

pub fn parse_submission_status(status: SubmissionStatus) -> String {
    match status {
        SubmissionStatus::Compiling => "compiling".to_string(),
        SubmissionStatus::Compiled => "compiled".to_string(),
        SubmissionStatus::Running(idx) => format!("running on test #{}", idx),
        SubmissionStatus::Done(_) => "Completed".to_string(),
        _ => "Running".to_string(),
    }
}

pub static PULL_MSG: &str = "in_queue";
