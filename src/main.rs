use clap::{Arg, Command};
use grep_rs::matcher;
use std::fs::File;
use std::io::prelude::*;
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Cli {
//     /// Name of the person to greet
//     file_name: String,

//     /// Regex expression
//     expr: String,
// }// 1.6.1

use colored::Colorize;
fn print_found_line(x: &i32, line: &str, found: &str) {
    let line_to_print = line.replace(found, &found.green().to_string());
    println!("[{}] {}", x.to_string().blue(), line_to_print);
}
const FILENAME: &str = "FILENAME";
const STRINGTOFIND: &str = "STRINGTOFIND";
const RECURSIVE: &str = "RECURSIVE";
const IGNORE_CASESENSITIVE: &str = "IGNORE";
fn main() -> std::io::Result<()> {
    // EASY TO IMPLEMENT OTHER FEATURES LATER
    let args = Command::new("GREPS")
        .version("1.0")
        .author("X")
        .about("Implementation of grep in rust")
        .arg(Arg::new(STRINGTOFIND))
        .arg(Arg::new(FILENAME).min_values(1))
        .arg(
            Arg::new(RECURSIVE)
                .long("recursive")
                .short('r')
                .help("Search recursively across dir"),
        )
        .arg(
            Arg::new(IGNORE_CASESENSITIVE)
                .long("ignore")
                .short('i')
                .help("ignore case SENSITIVE"),
        )
        .get_matches();
    let fileNames: Vec<_> = args.values_of(FILENAME).unwrap_or_default().collect();
    // Checking all filenames
    println!("{:?}", fileNames);

    // Check for all the values in between the value
    let mut _chk: String = String::from("*");
    let mut _c = String::from(args.get_one::<String>(STRINGTOFIND).unwrap());
    println!("{}", args.get_one::<String>("STRINGTOFIND").unwrap());

    _chk.push_str(_c.as_str());
    _chk.push('*');
    // let mut _file: File = File::open(args.get_one::<String>(FILENAME).unwrap())?;

    for file in fileNames {
        let mut contents = String::new();
        let mut _file: File = File::open(file.to_string())?;
        _file.read_to_string(&mut contents)?;
        let _line: Vec<_> = contents.split("\n").collect();
        let mut sentence_line = 1;

        for sentence in _line {
            if args.is_present(IGNORE_CASESENSITIVE) {
                if matcher::is_match_regex(
                    sentence.to_string().to_lowercase(),
                    _chk.to_string().to_lowercase(),
                ) {
                    print!("{} ", file.magenta());
                    print_found_line(&sentence_line, sentence, &_c);
                }
            } else {
                if matcher::is_match_regex(sentence.to_string(), _chk.to_string()) {
                    print!("{} ", file.magenta());
                    print_found_line(&sentence_line, sentence, &_c);
                }
            }

            sentence_line += 1;
        }
        println!("");
    }

    Ok(())
}
