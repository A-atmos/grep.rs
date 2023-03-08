use colored::Colorize;

use std::{io::prelude::*,fs::File, fs::create_dir, path::Path};
use super::trie::CharNode;
use sha2::{Sha256,Digest};


pub fn print_found_line(x: &i32, line: &str, found: &str) {
    let line_to_print = line.replace(found, &found.green().to_string());
    println!("[{}] {}", x.to_string().blue(), line_to_print);
}



pub fn file_present_or_create(path: String, case_insensitive: bool)-> CharNode{

    let mut trie_data  =  CharNode::new();

    // read from file and store the text in contents
    let mut file = File::open(path).expect("File not Found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    

    //calculate the checksum of the file
    let hash = Sha256::digest(contents.as_bytes());
    let mut st_hash = format!("{:x}",hash).to_owned();
    println!("{}",st_hash);

    st_hash = ".gs_cache/".to_string() + &st_hash;

    let path = Path::new(st_hash.as_str());

    if path.exists(){
        print!("Path Exists");
        let mut json_file = File::open(path).expect("msg");
        let mut json_content = String::new();
        json_file.read_to_string(&mut json_content).expect("msg");

        trie_data = serde_json::from_str(&json_content).unwrap();
    }
    else{
        print!("Path doesn't exist");
        let mut json_file : File;
        if Path::new(".gs_cache").is_dir(){
        }
        else{
            create_dir(".gs_cache").expect("msg");
        }
        json_file = File::create(&path).expect("msg");

        if case_insensitive {
            contents = contents.to_lowercase();
        }
        
        
        for _ in contents.split(' ').map(|word| trie_data.insert(word.trim())) {};
        let serialized = serde_json::to_string(&trie_data).unwrap();

        json_file.write_all(serialized.as_bytes()).expect("msg");


    }

    
    // println!("{trie_data:#?}");
    
    trie_data

}

pub fn serialized_file_present(path: String)->bool{
    let mut file = File::open(path).expect("File not Found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    

    //calculate the checksum of the file
    let hash = Sha256::digest(contents.as_bytes());
    let mut st_hash = format!("{:x}",hash).to_owned();
    println!("{}",st_hash);

    st_hash = ".gs_cache/".to_string() + &st_hash;

    let path = Path::new(st_hash.as_str());
    return path.exists();
}
