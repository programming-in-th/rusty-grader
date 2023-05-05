use grader::errors::GraderError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("submission not found")]
    SubmissionNotFound,
    #[error("submission is already judged")]
    AlreadyJudge,
    #[error("db error: {0}")]
    DbError(#[from] tokio_postgres::Error),
    #[error("decompress error: {0}")]
    DecompressError(#[from] std::io::Error),
    #[error("from utf-8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("json parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("expected int, found {0}")]
    InvalidSubmissionId(String),
    #[error("invalid code, expectd array")]
    InvalidCode,
    #[error("grader error: {0}")]
    GraderError(#[from] GraderError),
}
