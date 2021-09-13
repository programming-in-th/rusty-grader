use std::env;
use std::path::Path;

pub struct Instance {
  pub box_id: u64,
  pub bin_path: String,
  pub log_file: String,
  pub time_limit: f64,
  pub memory_limit: u64,
  pub input_path: String,
  pub output_path: String,
  pub runner_path: String
}

pub enum RunVerdict {
  OK,
  TLE,
  MLE,
  RE,
  XX,
  OT
}

pub struct InstanceResult {
  status: RunVerdict,
  time_usage: u64,
  memory_usage: u64 
}

impl Instance {
  pub fn get_arguments(&self) -> Result<Vec<String>, ()> {
    let mut args : Vec<String> = Vec::new();
    args.push(String::from("--cg"));
    args.push(String::from("--cg-timing"));
    args.push(String::from("--processes=128"));

    args.push(String::from("-b"));
    args.push(self.box_id.to_string());

    args.push(String::from("-M"));
    args.push(self.log_file.clone());

    args.push(String::from("-t"));
    args.push(self.time_limit.to_string());

    args.push(format!("--cg-mem={}", self.memory_limit));

    args.push(String::from("-w"));
    args.push((self.time_limit + 5.0).to_string());

    args.push(String::from("-x"));
    args.push((self.time_limit + 1.0).to_string());

    let alternative_path = env::var("ALTERNATIVE_PATH").expect("Expect to get alternative path from env");
    if Path::new(&alternative_path).is_dir() {
      args.push(format!("--dir={}", alternative_path));
    }

    args.push(String::from("-i"));
    args.push(self.input_path.clone());

    args.push(String::from("-o"));
    args.push(self.output_path.clone());

    Ok(args)
  } 

  pub fn init() -> Result<(), ()> {
    Ok(())
  }

  pub fn run() -> Result<InstanceResult, ()> {
    Err(())
  }
  
  pub fn cleanup() -> Result<(), ()> {
    Ok(())
  }
}