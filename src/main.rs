
use std::fs::File;
use clap::Parser;
use std::io::prelude::*;
mod lib;


#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    file_name: String,

    /// Regex expression
    expr: String,

}


fn main()-> std::io::Result<()> {

    let args = Cli::parse();

    let mut _file: File = File::open(args.file_name) ?;

    let mut contents = String::new();

    _file.read_to_string(&mut contents)?;

    let _line: Vec<_> = contents.split("\n").collect();


    // Check for all the values in between the value
    let mut _chk : String = String::from("*");
    let mut _c = String::from(args.expr);
    _chk.push_str(_c.as_str());
    _chk.push('*');



    for sentence in _line{
        
        if lib::is_match(sentence.to_string(),_chk.to_string()){
            println!("{}\n",sentence);
        }
    }



    Ok(())

}
