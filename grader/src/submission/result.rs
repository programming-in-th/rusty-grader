#[derive(Default, Debug, PartialEq, Clone)]
pub struct RunResult {
    pub submission_id: String,
    pub test_index: u64,
    pub status: String,
    pub time_usage: f64,
    pub memory_usage: u64,
    pub score: f64,
    pub message: String,
}

impl RunResult {
    pub fn from(submission_id: String, index: u64, time_usage: f64, memory_usage: u64) -> Self {
        RunResult {
            submission_id,
            test_index: index,
            time_usage,
            memory_usage,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct GroupResult {
    pub score: f64,
    pub full_score: u64,
    pub submission_id: String,
    pub group_index: u64,
    pub run_result: Vec<RunResult>,
}

impl GroupResult {
    pub fn from(full_score: u64, submission_id: String, index: u64) -> Self {
        GroupResult {
            full_score,
            submission_id,
            group_index: index,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct SubmissionResult {
    pub score: f64,
    pub full_score: u64,
    pub submission_id: String,
    pub group_result: Vec<GroupResult>,
}
