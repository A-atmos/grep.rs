use std::fs::File;
use std::io::prelude::*;

use colored::Colorize;

use grep_rs::matcher;
use grep_rs::argument::_parse_args;

const FILENAME: &str = "FILENAME";
const STRINGTOFIND: &str = "STRINGTOFIND";
const RECURSIVE: &str = "RECURSIVE";
const IGNORE_CASESENSETIVE: &str = "IGNORE";


fn print_found_line(x: &i32, line: &str, found: &str) {
    let line_to_print = line.replace(found, &found.green().to_string());
    println!("[{}] {}", x.to_string().blue(), line_to_print);
}


fn main() -> std::io::Result<()> {


    // EASY TO IMPLEMENT OTHER FEATURES LATER
    let args = _parse_args();

    let file_names: Vec<_> = args.values_of(FILENAME).unwrap_or_default().collect();
    // Checking all filenames
    println!("{:?}", file_names);

    // Check for all the values in between the value
    let mut _chk: String = String::from("*");
    let mut _c = String::from(args.get_one::<String>(STRINGTOFIND).unwrap());
    println!("{}", args.get_one::<String>("STRINGTOFIND").unwrap());

    _chk.push_str(_c.as_str());
    _chk.push('*');

    for file in file_names {
        let mut contents = String::new();
        let mut _file: File = File::open(file.to_string())?;
        _file.read_to_string(&mut contents)?;
        let _line: Vec<_> = contents.split("\n").collect();
        let mut sentence_line = 1;

        for sentence in _line {
            if args.is_present(IGNORE_CASESENSETIVE) {
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
