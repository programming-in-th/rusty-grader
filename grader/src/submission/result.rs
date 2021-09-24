#[derive(Default, Debug, PartialEq)]
pub struct RunResult<'a> {
    pub submission_id: &'a str,
    pub test_index: u64,
    pub status: String,
    pub time_usage: f64,
    pub memory_usage: u64,
    pub score: f64,
    pub message: String,
}

impl<'a> RunResult<'a> {
    pub fn from(submission_id: &'a str, index: u64, time_usage: f64, memory_usage: u64) -> Self {
        RunResult {
            submission_id,
            test_index: index,
            time_usage,
            memory_usage,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct GroupResult<'a> {
    pub score: f64,
    pub full_score: u64,
    pub submission_id: &'a str,
    pub group_index: u64,
    pub run_result: Vec<RunResult<'a>>,
}

impl<'a> GroupResult<'a> {
    pub fn from(full_score: u64, submission_id: &'a str, index: u64) -> Self {
        GroupResult {
            full_score,
            submission_id,
            group_index: index,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct SubmissionResult<'a> {
    pub score: f64,
    pub full_score: u64,
    pub submission_id: &'a str,
    pub group_result: Vec<GroupResult<'a>>,
}
