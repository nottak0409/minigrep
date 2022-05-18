extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // 引数解析時に問題
        eprintln!("引数に問題があります: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("エラーが発生しました {}", e);
        process::exit(1);
    }
}