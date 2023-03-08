use colored::Colorize;

use std::{io::prelude::*,fs::File, fs::create_dir, path::Path, io};
use super::trie::CharNode;
use sha2::{Sha256,Digest};


pub fn print_found_line(x: &i32, line: &str, found: &str) {
    let line_to_print = line.replace(found, &found.green().to_string());
    println!("[{}] {}", x.to_string().blue(), line_to_print);
}



pub fn file_present_or_create(path: String, case_insensitive: bool)-> CharNode{

    let mut trie_data  =  CharNode::new();

    // read from file and store the text in contents
    let file = File::open(path).expect("File not Found");

    let content = io::BufReader::new(file);


    //calculate the checksum of the file
    let hash = Sha256::digest(content.buffer());
    let mut st_hash = format!("{:x}",hash).to_owned();

    st_hash = ".gs_cache/".to_string() + &st_hash;

    let path = Path::new(st_hash.as_str());

    if path.exists(){
        // Path Exists
        let mut json_file = File::open(path).expect("Unable to create json file");
        let mut json_content = String::new();
        json_file.read_to_string(&mut json_content).expect("Unable to read file");

        trie_data = serde_json::from_str(&json_content).unwrap();
    }
    else{
        // path doesnot exist
        let mut json_file : File;
        if Path::new(".gs_cache").is_dir(){
        }
        else{
            create_dir(".gs_cache").expect("Unable to create directory!");
        }
        json_file = File::create(&path).expect("Unable to create json file");

    
        
        let _lines = content.lines();

        for (idx,str) in _lines.enumerate(){

            if let Ok(mut item) = str{

                if case_insensitive{
                    item = item.to_lowercase();
                }
                for _ in item.split(' ').map(|word| trie_data.insert(word.trim(),idx)) {};
            }
            
        }

        
        let serialized = serde_json::to_string(&trie_data).unwrap();

        json_file.write_all(serialized.as_bytes()).expect("Unable to write");


    }

    
    // println!("{trie_data:#?}");
    
    trie_data

}

pub fn serialized_file_present(path: String)->bool{
    let file = File::open(path).expect("File not Found");

    let content = io::BufReader::new(file);


    //calculate the checksum of the file
    let hash = Sha256::digest(content.buffer());
    let mut st_hash = format!("{:x}",hash).to_owned();

    st_hash = ".gs_cache/".to_string() + &st_hash;

    let path = Path::new(st_hash.as_str());
    return path.exists();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}