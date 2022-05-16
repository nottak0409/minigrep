use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("ファイルが見つかりませんでした");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("ファイルの読み込み中に問題が発生しました");
    println!("テキストは{}", contents);
}

struct Config {
    query: String,
    filename: String,
}
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}