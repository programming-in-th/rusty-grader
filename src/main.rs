#[allow(dead_code)]
mod instance;
use dotenv::dotenv;
use std::path::PathBuf;

struct Fff(u32);

fn main() {
    dotenv().ok();
    let x = Fff(10);
    println!("{}", x.0);
}
