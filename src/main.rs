use colored::Colorize;
use grep_rs::{matcher,
            KMP,
            argument::_parse_args,
            dlf::{print_found_line, serialized_file_present},
            dlf::file_present_or_create,
            trie::{CharNode}};
use std::{fs,fs::{File},io::prelude::*,io, vec};
use std::collections::HashSet;
use walkdir::WalkDir;

// Program begins here
fn main() -> std::io::Result<()> {
    // EASY TO IMPLEMENT OTHER FEATURES LATER
    let args = _parse_args();


    // check if a single file or multiple files
    let mut  _file_names: Vec<_> = vec![];
    let mut current_files: Vec<String> = vec![];


    if args.is_present("FILENAME")
    {
        // from file/files
        _file_names = args.values_of("FILENAME").unwrap_or_default().collect();
    }



    // processing filename for multiple files
    if args.is_present("RECURSIVE") {
        for e in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            if e.metadata().unwrap().is_file() {
                if e.path().display().to_string().contains("txt") {
                    current_files.push(e.path().display().to_string());
                }
            }
        }
    } else if args.is_present("CURRENT_DIRECTORY") {
        // For current directory
        let paths = fs::read_dir("./").unwrap();

        for mut path in paths {
            if path
                .as_mut()
                .unwrap()
                .path()
                .display()
                .to_string()
                .contains("txt")
            {
                current_files.push(path.unwrap().path().display().to_string());
            }

            // let _filep = File::open(path.unwrap().path().display().to_string()).expect("Hello");
        }
    }


    // To convert simply to &str from String type
    let v2: Vec<&str> = current_files.iter().map(|s| &**s).collect();



    if args.is_present("PREPROCESSOR")
    {
        // preprocessing the text if flag provided and serializing it into trie
        for file in if args.is_present("CURRENT_DIRECTORY") || args.is_present("RECURSIVE") {
            v2
        } else {
            _file_names
        }
        {
            file_present_or_create(file.to_string(), args.is_present("IGNORE"));
        }
    }
    else{

        let mut _c = String::from(args.get_one::<String>("STRINGTOFIND").unwrap());
        let mut _chk: String = String::from("*");
        if args.is_present("HAS_REGEX"){
            _chk.push_str(_c.as_str());
            _chk.push('*');
        
        }
        
        // Check whether CURRENT_DIRECTORY flag -r is given or not
        // If user inputs both -r as well as current directory flag (-c) then -r will be selected by default
        
        if args.is_present("IGNORE"){
            _c = _c.to_lowercase();
            _chk = _chk.to_string().to_lowercase();
        }
        else{
            _chk = _chk.to_string();
        }


        if args.is_present("FILENAME") || args.is_present("CURRENT_DIRECTORY") || args.is_present("RECURSIVE"){
            // if search in a file
            for file in if args.is_present("CURRENT_DIRECTORY") || args.is_present("RECURSIVE") {
                v2
            } else {
                _file_names
            } {
        
                if args.is_present("HAS_REGEX") {
                    let mut contents = String::new();
                    let mut _file: File = File::open(file.to_string())?;
                    _file.read_to_string(&mut contents)?;
                    let _line: Vec<_> = contents.split("\n").collect();
                    let mut sentence_line = 1;
                    // the string literal is a regex pattern
                    let mut _sentence: String;
                    for sentence in _line {
                        if args.is_present("IGNORE") 
                        {
                            _sentence = sentence.to_string().to_lowercase();
                        }
                        else{
                            _sentence = sentence.to_string();
                        }
        
                        if matcher::is_match_regex(_sentence, _chk.to_string()) {
                            print!("{} ", file.magenta());
                            print_found_line(&sentence_line, sentence, &_c);
                        }
        
                        sentence_line += 1;
                    }
                }
                else {
                    if args.is_present("WORD"){
                        if serialized_file_present(file.to_string()){
                            let mut trie_ds : CharNode = file_present_or_create(
                                                            file.to_string(),
                                                            args.is_present("IGNORE")
                                                            );
                            
                            // just prints if the string is present or not 

                            let res = trie_ds.search(_c.as_str()); 
                            if res.0 {
                                // result present
                                let reader = io::BufReader::new(File::open(file.to_string()).expect("Cannot open file"));
                                let mut lines = reader.lines().map(|l|l.unwrap());
                                
                                let mut dif = 0;
                                // only unique elements
                                let _lines = res.1.into_iter()
                                .collect::<HashSet<_>>()
                                .into_iter();
                                
                                let mut sorted_lines: Vec<usize> = vec![];

                                for _line_no in _lines{
                                    sorted_lines.push(_line_no);
                                }

                                sorted_lines.sort();

                                for _line_no in sorted_lines{

                                    let val = lines.nth(_line_no-dif).expect("msg");
                                    dif = _line_no+1;
                                    print_found_line(&(_line_no as i32 +1), &val, &_c);
                                }

                            }                        
                        }
                        else{
                            let mut contents = String::new();
                            let mut _file: File = File::open(file.to_string())?;
                            _file.read_to_string(&mut contents)?;
                            let _line: Vec<_> = contents.split("\n").collect();
                            let mut sentence_line = 1;
                            // the string literal is a word to search
                            let kmp = KMP::new(&_c);
                            let mut _sentence: String;
            
                            for sentence in _line {
                                
                                if args.is_present("IGNORE") 
                                {
                                    _sentence = sentence.to_string().to_lowercase();
                                }
                                else{
                                    _sentence = sentence.to_string();
                                }
                                if sentence == "" {
                                    continue;
                                }
                                if kmp.index_of_any(&_sentence) == -1 {
                                    sentence_line += 1;
                                    continue;
                                } else {
                                    print!("{} ", file.magenta());
                                    print_found_line(&sentence_line, &sentence, &_c);
                                }
                                sentence_line += 1;
                                
                            }
                        }
                    }
                    else{
                        let mut contents = String::new();
                            let mut _file: File = File::open(file.to_string())?;
                            _file.read_to_string(&mut contents)?;
                            let _line: Vec<_> = contents.split("\n").collect();
                            let mut sentence_line = 1;
                            // the string literal is a word to search
                            let kmp = KMP::new(&_c);
                            let mut _sentence: String;
            
                            for sentence in _line {
                                
                                if args.is_present("IGNORE") 
                                {
                                    _sentence = sentence.to_string().to_lowercase();
                                }
                                else{
                                    _sentence = sentence.to_string();
                                }
                                if sentence == "" {
                                    continue;
                                }
                                if kmp.index_of_any(&_sentence) == -1 {
                                    sentence_line += 1;
                                    continue;
                                } else {
                                    print!("{} ", file.magenta());
                                    print_found_line(&sentence_line, &sentence, &_c);
                                }
                                sentence_line += 1;
                                
                            }
                    }
                }
            }
        }
        else{
            // read from stdin
            let stdin = io::stdin();
            let lines = stdin.lock().lines();



            if args.is_present("HAS_REGEX") {
                // if regex is present with stdin
                let mut sentence_line = 1;
                // the string literal is a regex pattern
                let mut _sentence: String;
                for line in lines {
                    let sentence = line.expect("Couldnot read line from stdin.");
                    if args.is_present("IGNORE") 
                    {
                        _sentence = sentence.to_string().to_lowercase();
                    }
                    else{
                        _sentence = sentence.to_string();
                    }
    
                    if matcher::is_match_regex(_sentence, _chk.to_string()) {
                        print_found_line(&sentence_line, sentence.as_str(), &_c);
                    }
    
                    sentence_line += 1;
                }
            }
            else{
                
                let mut sentence_line = 1;
                // the string literal is a word to search
                let kmp = KMP::new(&_c);
                let mut _sentence: String;

                for line in lines {
                    let sentence = line.expect("Error reading input from stdin");
                    if args.is_present("IGNORE") 
                    {
                        _sentence = sentence.to_string().to_lowercase();
                    }
                    else{
                        _sentence = sentence.to_string();
                    }
                    if sentence == "" {
                        continue;
                    }
                    if kmp.index_of_any(&_sentence) == -1 {
                        sentence_line += 1;
                        continue;
                    } else {
                        print_found_line(&sentence_line, &sentence, &_c);
                    }
                    sentence_line += 1;
                    
                }
            }

        }
    }    
    

    Ok(())
}
