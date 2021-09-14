#[derive(Debug, PartialEq)]
pub enum InstanceError {
    PermissionError(String),
    EnvironmentError(String),
}
