#[derive(Debug, PartialEq)]
pub enum InstanceError {
    PermissionError(&'static str),
    EnvironmentError(String),
}
