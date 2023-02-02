use std::env;
use std::process::ExitCode;

use minigrep::Config;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let config = Config::create(&args);

    match config {
        Err(err_msg) => {
            eprintln!("{err_msg}");
            ExitCode::from(ExitCode::FAILURE)
        }
        Ok(config) => ExitCode::from(minigrep::do_grep(config)),
    }
}
