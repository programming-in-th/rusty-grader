#[allow(dead_code)]
mod instance;
use dotenv::dotenv;
use instance::model::{ Instance };

fn main() {
    dotenv().ok();
    let ins = Instance {
        box_id: 1,
        bin_path: String::from("/hello/world"),
        log_file: String::from("/log/file"),
        time_limit: 1.0,
        memory_limit: 512000,
        input_path: String::from("/path/to/in"),
        output_path: String::from("/path/to/out"),
        runner_path: String::from("/path/to/runner")
    };
    let args = ins.get_arguments().unwrap();
    for st in args.iter() {
        println!("{}", st);
    }
}
