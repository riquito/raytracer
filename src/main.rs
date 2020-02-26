use raytracer::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if let Err(e) = run() {
        panic!("Whops: {:?}", e);
    }
}
