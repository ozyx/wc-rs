extern crate regex;
use regex::Regex;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

// Box<dyn Error> == "a type that implements the Error trait"
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let count = count(config);
    println!("{}", count.unwrap());
    Ok(())
}

pub fn count(config: Config) -> Result<String, Box<dyn Error>>
// where
// T: fmt::Display,
{
    let contents = fs::read_to_string(&config.filename)?;
    match config.mode {
        CountMode::Bytes => Ok(format!("{}\t{}", contents.bytes().count(), config.filename)),
        CountMode::Lines => Ok(format!("{}\t{}", contents.lines().count(), config.filename)),
        CountMode::Characters => Ok(format!("{}\t{}", contents.chars().count(), config.filename)),
        CountMode::Words => Ok(format!("{}\t{}", count_words(&contents), config.filename)),
        CountMode::All => Ok(format!(
            "{}\t{}\t{}\t{}",
            contents.lines().count(),
            count_words(&contents),
            contents.chars().count(),
            config.filename
        )),
    }
}

pub fn count_words<'a>(contents: &'a str) -> usize {
    let mut count: usize = 0;

    for line in contents.lines() {
        for _word in line.split_whitespace() {
            count += 1;
        }
    }

    count
}

pub enum CountMode {
    Bytes,
    Lines,
    Characters,
    Words,
    All,
}

pub struct Config {
    filename: String,
    mode: CountMode,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let len = args.len();
        if len < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        if len == 2 {
            return Ok(Config {
                filename: filename,
                mode: CountMode::All,
            });
        }

        match args[2].as_str() {
            "-c" => Ok(Config {
                filename: filename,
                mode: CountMode::Characters,
            }),
            "-w" => Ok(Config {
                filename: filename,
                mode: CountMode::Words,
            }),
            "-l" => Ok(Config {
                filename: filename,
                mode: CountMode::Lines,
            }),
            "-b" => Ok(Config {
                filename: filename,
                mode: CountMode::Bytes,
            }),
            _ => Ok(Config {
                filename: filename,
                mode: CountMode::All,
            }),
        }
    }
}
