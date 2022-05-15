use std::env;

fn main() {
    let args: Vec<String> = env::args().collection();
    println!("{:?}", args);
}
