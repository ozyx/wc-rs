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

pub fn count(config: Config) -> Result<FileStats, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    match config.mode {
        CountMode::Bytes => Ok(FileStats {
            bytes: Some(contents.bytes().count()),
            lines: None,
            chars: None,
            words: None,
            filename: config.filename,
        }),
        CountMode::Lines => Ok(FileStats {
            bytes: None,
            lines: Some(contents.lines().count()),
            chars: None,
            words: None,
            filename: config.filename,
        }),
        CountMode::Characters => Ok(FileStats {
            bytes: None,
            lines: None,
            chars: Some(contents.chars().count()),
            words: None,
            filename: config.filename,
        }),
        CountMode::Words => Ok(FileStats {
            bytes: None,
            lines: None,
            chars: None,
            words: Some(count_words(&contents)),
            filename: config.filename,
        }),
        CountMode::All => Ok(FileStats {
            bytes: None,
            lines: Some(contents.lines().count()),
            chars: Some(contents.chars().count()),
            words: Some(count_words(&contents)),
            filename: config.filename,
        }),
    }
}

pub fn count_words<'a>(contents: &'a str) -> usize {
    let mut count: usize = 0;

    for line in contents.lines() {
        count += line.split_ascii_whitespace().count();
    }

    count
}

pub struct FileStats {
    filename: String,
    bytes: Option<usize>,
    lines: Option<usize>,
    chars: Option<usize>,
    words: Option<usize>,
}

impl Default for FileStats {
    fn default() -> Self {
        Self {
            filename: String::new(),
            bytes: None,
            lines: None,
            chars: None,
            words: None,
        }
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for FileStats {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match (self.bytes, self.lines, self.chars, self.words) {
            (Some(bytes), None, None, None) => write!(f, "{}\t{}", bytes, self.filename),
            (None, Some(lines), None, None) => write!(f, "{}\t{}", lines, self.filename),
            (None, None, Some(chars), None) => write!(f, "{}\t{}", chars, self.filename),
            (None, None, None, Some(words)) => write!(f, "{}\t{}", words, self.filename),
            (None, Some(lines), Some(chars), Some(words)) => {
                write!(f, "{}\t{}\t{}\t{}", lines, words, chars, self.filename)
            }
            (_, _, _, _) => panic!("This should never happen! Contact the developer immediately"),
        }
    }
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
            "-m" => Ok(Config {
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
            "-c" => Ok(Config {
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
