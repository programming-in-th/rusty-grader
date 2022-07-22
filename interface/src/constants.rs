use grader::submission::SubmissionStatus;

pub static PULL_MSG: &str = "In Queue";
pub static ERROR_MSG: &str = "Judge Error";

pub fn parse_submission_status(status: SubmissionStatus) -> String {
    match status {
        SubmissionStatus::Compiling => "Compiling".to_string(),
        SubmissionStatus::Compiled => "Compiled".to_string(),
        SubmissionStatus::CompilationError(_) => "Compilation Error".to_string(),
        SubmissionStatus::Running(idx) => format!("Running on test #{}", idx),
        SubmissionStatus::Done(_) => "Completed".to_string(),
        _ => ERROR_MSG.to_string(),
    }
}
