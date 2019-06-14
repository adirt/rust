use std::env;
use std::process;
use sopcast::run;
use sopcast::config::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error_message| {
        eprintln!("Usage: sopcast <channel>\n\n{}", error_message);
        process::exit(1);
    });
    if let Err(error_message) = run(&config) {
        eprintln!("SopCast error: {}", error_message);
        process::exit(1);
    }
    process::exit(0);
}
