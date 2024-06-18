use clap::{App, Arg};

fn main() {
    let _matches
        = App::new("echor")
                .version("0.1.0")
                .author("SE15 <se15@kmitl.ac.th")
                .about("Rust echo")
                .arg(
                    Arg::with_name("text")
                        .value_name("TEXT")
                        .help("Input text")
                        .required(true)
                        .min_values(1),
                )
                .arg(
                    Arg::with_name("omit_newline")
                        .short("n")
                        .help("Do not print newline")
                        .takes_value(false),
                )
                .get_matches();

            let text = _matches.values_of_lossy("text").unwrap();

            println!("Hello, {}!", text.join(" "));
}


