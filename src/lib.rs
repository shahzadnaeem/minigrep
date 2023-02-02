use std::env;
use std::fs;
use std::process::ExitCode;

pub fn show_args(args: &[String]) -> () {
    args.iter().enumerate().for_each(|(i, a)| {
        if i != 0 {
            println!("{}: {}", i, a);
        }
    });
}

pub struct Config {
    search: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn create(args: &[String]) -> Result<Config, &'static str> {
        const MIN_ARGS: usize = 2;

        if args.len() < MIN_ARGS + 1 {
            // println!("ERROR: Not enough arguments - need: 'search string' 'file'");
            // show_args(&args);
            Err("ERROR: Not enough arguments - need: 'search string' 'file'")
        } else {
            let ignore_case = env::var("NC").is_ok();

            Ok(Config {
                search: args[1].clone(),
                file_path: args[2].clone(),
                ignore_case,
            })
        }
    }
}

pub fn do_grep(config: Config) -> ExitCode {
    let file_contents = fs::read_to_string(&config.file_path);

    match file_contents {
        Err(_) => {
            eprintln!("ERROR: Unable to read file '{}'", &config.file_path);
            ExitCode::FAILURE
        }

        Ok(file_contents) => {
            // println!("{}\n{}", config.file_path, file_contents);
            let result = if config.ignore_case {
                search_for_nc(&config.search, &file_contents)
            } else {
                search_for(&config.search, &file_contents)
            };

            if result.len() > 0 {
                result.into_iter().for_each(|line| println!("{line}"));
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
    }
}

pub fn search_for<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    Vec::from_iter(contents.lines().filter(|line| line.contains(search)))
}

pub fn search_for_nc<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    Vec::from_iter(
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&search.to_lowercase())),
    )
}

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
