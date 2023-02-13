#![allow(dead_code)]
use colored::Colorize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use grep_rs::argument::_parse_args;
use grep_rs::{matcher, KMP};

use std::time::Instant;

const FILENAME: &str = "FILENAME";
const STRINGTOFIND: &str = "STRINGTOFIND";
const RECURSIVE: &str = "RECURSIVE";
const IGNORE_CASESENSETIVE: &str = "IGNORE";
const HAS_REGEX: &str = "HAS_REGEX";
const CURRENT_DIRECTORY: &str = "CURRENT_DIRECTORY";

fn print_found_line(x: &i32, line: &str, found: &str) {
    let line_to_print = line.replace(found, &found.green().to_string());
    println!("[{}] {}", x.to_string().blue(), line_to_print);
}

fn main() -> std::io::Result<()> {
    // EASY TO IMPLEMENT OTHER FEATURES LATER
    let args = _parse_args();

    let mut file_names: Vec<_> = args.values_of(FILENAME).unwrap_or_default().collect();
    // Checking all filenames
    println!("{:?}", file_names);

    // Check for all the values in between the value
    let mut _chk: String = String::from("*");
    let mut _c = String::from(args.get_one::<String>(STRINGTOFIND).unwrap());
    println!("{}", args.get_one::<String>("STRINGTOFIND").unwrap());

    _chk.push_str(_c.as_str());
    _chk.push('*');
    let mut currentFiles: Vec<String> = vec![];
    // Check whether CURRENT_DIRECTORY flag -c is given or not
    if args.is_present(CURRENT_DIRECTORY) {
        let mut paths = fs::read_dir("./").unwrap();

        for mut path in paths {
            if path
                .as_mut()
                .unwrap()
                .path()
                .display()
                .to_string()
                .contains("txt")
            {
                currentFiles.push(path.unwrap().path().display().to_string());
            }

            // let _filep = File::open(path.unwrap().path().display().to_string()).expect("Hello");
        }
    }

    let v2: Vec<&str> = currentFiles.iter().map(|s| &**s).collect();
    println!("{:?}", v2);
    for file in if args.is_present(CURRENT_DIRECTORY) {
        v2
    } else {
        file_names
    } {
        let mut contents = String::new();
        let mut _file: File = File::open(file.to_string())?;
        _file.read_to_string(&mut contents)?;
        let _line: Vec<_> = contents.split("\n").collect();
        let mut sentence_line = 1;

        if args.is_present(HAS_REGEX) {
            // the string literal is a regex pattern

            let before = Instant::now();
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

            println!("Elapsed time: {:.2?}", before.elapsed());
        } else {
            // the string literal is a word to search

            if args.is_present(IGNORE_CASESENSETIVE) {
                let kmp = KMP::new(&_c.to_lowercase());
                let before = Instant::now();
                for sentence in _line {
                    if sentence == "" {
                        continue;
                    }
                    if kmp.index_of_any(&sentence.to_lowercase()) == -1 {
                        continue;
                    } else {
                        print!("{} ", file.magenta());
                        print_found_line(&sentence_line, sentence, &_c);
                    }
                    sentence_line += 1;
                }
                println!("Elapsed time: {:.2?}", before.elapsed());
            } else {
                let kmp = KMP::new(&_c);

                let before = Instant::now();
                for sentence in _line {
                    if sentence == "" {
                        continue;
                    }
                    if kmp.index_of_any(&sentence) == -1 {
                        continue;
                    } else {
                        print!("{} ", file.magenta());
                        print_found_line(&sentence_line, sentence, &_c);
                    }
                    sentence_line += 1;
                }
                println!("Elapsed time: {:.2?}", before.elapsed());
            }
        }
    }

    Ok(())
}
