#[derive(Debug)]
pub enum InstanceError {
    PermissionError(String),
    EnvorimentError(String),
}
