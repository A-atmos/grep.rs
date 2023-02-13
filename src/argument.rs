use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, ArgMatches};


pub fn _parse_args() -> ArgMatches {
    App::new("grep-rs")
		.version("0.1.4")
		.author("Aavash Chhetri <aavashchhetri01@gmail.com>,
				Mukti Subedi <077bct048.mukti@pcampus.edu.np>, 
				Kushal Poudel <077bct039.kushal@pcampus.edu.np>")
		.about("A tool to search files")
		.setting(ArgRequiredElseHelp)
		.args(&[
			Arg::new("STRINGTOFIND"),
			Arg::new("FILENAME").min_values(1),
			Arg::new("file")
				.help("Target file sepearated with (\\n)!")
				.short('f')
				.long("file")
				.takes_value(true),
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