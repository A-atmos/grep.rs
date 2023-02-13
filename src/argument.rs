use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, ArgMatches};


pub fn _parse_args() -> ArgMatches {
    App::new("grep-rs")
		.version("0.1.0")
		.author("Authors: 
        Aavash Chhetri <aavashchhetri01@gmail.com> 
        Mukti Subedi <077bct048.mukti@pcampus.edu.np> 
        Kushal Poudel <077bct039.kushal@pcampus.edu.np>")
		.about("About: 
        A tool not to just search files but much more.")
		.setting(ArgRequiredElseHelp)
		.args(&[
			Arg::new("STRINGTOFIND"),
			Arg::new("FILENAME").min_values(1),
			Arg::new("RECURSIVE")
                .long("recursive")
                .short('r')
                .help("Search recursively across dir"),
			Arg::new("IGNORE")
                .long("ignore")
                .short('i')
                .help("ignore case sensetive"),
			
		])
		.get_matches()
}