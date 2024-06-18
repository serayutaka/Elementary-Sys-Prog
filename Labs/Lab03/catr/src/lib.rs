use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                let mut i = 0;
                for line_result in file.lines() {
                    let line = line_result?;
                    i = i + 1;
                    if config.number_lines{
                        println!("{:>6}\t{}", i, line);
                    }
                    else if config.number_nonblank_lines {
                        if line.is_empty() == false {
                            last_num = last_num + 1;
                            println!("{:>6}\t{}", last_num, line);
                        }
                        else {
                            print!("\n")
                        }
                    }
                    else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches
        = App::new("echor")
                .version("0.1.0")
                .author("SE15 <se15@kmitl.ac.th>")
                .about("Rust echo")
                .arg(
                    Arg::with_name("files")
                        .value_name("FILE")
                        .help("Input file(s)")
                        .multiple(true)
                        .default_value("-")
                )
                .arg(
                    Arg::with_name("number")
                        .short("n")
                        .long("number")
                        .help("Number lines")
                        .takes_value(false)
                        .conflicts_with("number_nonblank"),
                )
                .arg(
                    Arg::with_name("number_nonblank")
                        .short("b")
                        .long("number-nonblank")
                        .help("Number non-blank lines")
                        .takes_value(false),
                )
                .get_matches();

            let files = matches.values_of_lossy("files").unwrap();
            let bool1 = matches.is_present("number");
            let bool2 = matches.is_present("number_nonblank");

            Ok(Config {
                files: files,
                number_lines: bool1,
                number_nonblank_lines: bool2,
            })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new( BufReader::new(io::stdin()) )),
        _ => Ok(Box::new( BufReader::new(File::open(filename)?) )),
    }
}