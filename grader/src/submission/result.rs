#[derive(Debug, PartialEq)]
pub enum TestCaseVerdict {
    VerdictCorrect,
    VerdictIncorrect,
    VerdictPCorrect,
    VerdictSkip,
    VerdictTLE,
    VerdictMLE,
    VerdictRE,
    VerdictXX,
    VerdictSG,
}

#[derive(Default, Debug, PartialEq)]
pub struct RunResult {
    pub submission_id: String,
    pub test_index: u64,
    pub status: TestCaseVerdict,
    pub time_usage: f64,
    pub memory_usage: u64,
    pub score: f64,
    pub message: String,
}

impl Default for TestCaseVerdict {
    fn default() -> Self {
        TestCaseVerdict::VerdictSkip
    }
}

impl RunResult {
    pub fn from(submission_id: &str, index: u64) -> Self {
        let mut run_result: RunResult = Default::default();
        run_result.submission_id = submission_id.to_owned();
        run_result.test_index = index;

        run_result
    }
}

#[derive(Default, Debug)]
pub struct GroupResult {
    pub score: f64,
    pub full_score: u64,
    pub submission_id: String,
    pub group_index: u64,
    pub run_result: Vec<RunResult>,
}

impl GroupResult {
    pub fn from(full_score: u64, submission_id: &str, index: u64) -> Self {
        let mut group_result: GroupResult = Default::default();
        group_result.full_score = full_score;
        group_result.submission_id = submission_id.to_owned();
        group_result.group_index = index;

        group_result
    }
}

#[derive(Default, Debug)]
pub struct SubmissionResult {
    pub score: f64,
    pub full_score: u64,
    pub submission_id: String,
    pub group_result: Vec<GroupResult>,
}
