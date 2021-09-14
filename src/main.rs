#[allow(dead_code)]
#[allow(unused_variables)]
mod instance;
use dotenv::dotenv;
use instance::model::Instance;
use std::path::PathBuf;

#[allow(unused_variables)]
fn main() {
    dotenv().ok();
    let ins = Instance {
        box_id: 1,
        bin_path: PathBuf::from("/hello/world"),
        time_limit: 1.0,
        memory_limit: 512000,
        input_path: PathBuf::from("/path/to/in"),
        output_path: PathBuf::from("/path/to/out"),
        runner_path: PathBuf::from("/path/to/runner"),
        ..Default::default()
    };
}
