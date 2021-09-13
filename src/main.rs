#[allow(dead_code)]
#[allow(unused_variables)]
mod instance;
use dotenv::dotenv;
use instance::model::Instance;
use std::path::PathBuf;

#[allow(unused_variables)]
fn main() {
    dotenv().ok();
    let ins = Instance::new(
        1,
        PathBuf::from("/hello/world"),
        PathBuf::from("/log/file"),
        1.0,
        512000,
        PathBuf::from("/path/to/in"),
        PathBuf::from("/path/to/out"),
        PathBuf::from("/path/to/runner"),
    );
}
