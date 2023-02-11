use clap::{Arg, Command};
mod lib;
use std::fs::File;
use std::io::prelude::*;
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Cli {
//     /// Name of the person to greet
//     file_name: String,

//     /// Regex expression
//     expr: String,
// }

const FILENAME: &str = "FILENAME";
const STRINGTOFIND: &str = "STRINGTOFIND";
const RECURSIVE: &str = "RECURSIVE";
fn main() -> std::io::Result<()> {
    // EASY TO IMPLEMENT OTHER FEATURES LATER
    let args = Command::new("GREPS")
        .version("1.0")
        .author("X")
        .about("Implementation of grep in rust")
        .arg(Arg::new(STRINGTOFIND))
        .arg(Arg::new(FILENAME))
        .arg(
            Arg::new(RECURSIVE)
                .long("recursive")
                .short('r')
                .help("Search recursively across dir"),
        )
        .get_matches();

    let mut _file: File = File::open(args.get_one::<String>(FILENAME).unwrap())?;
    let mut contents = String::new();

    _file.read_to_string(&mut contents)?;

    let _line: Vec<_> = contents.split("\n").collect();

    // Check for all the values in between the value
    let mut _chk: String = String::from("*");
    let mut _c = String::from(args.get_one::<String>(STRINGTOFIND).unwrap());
    println!("{}", args.get_one::<String>("STRINGTOFIND").unwrap());
    _chk.push_str(_c.as_str());
    _chk.push('*');
    let mut sentence_line = 0;
    for sentence in _line {
        if lib::is_match(sentence.to_string(), _chk.to_string()) {
            print!("Line: {} ", sentence_line);
            println!("{}\n", sentence);
        }
        sentence_line += 1;
    }

    Ok(())
}
