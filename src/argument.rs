use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, ArgMatches};

pub fn _parse_args() -> ArgMatches {
    App::new("grep-rs")
        .version("0.1.0")
        .author(
            "Authors: 
        Aavash Chhetri <077bct004.aavash@pcampus.edu.np> 
        Mukti Subedi <077bct048.mukti@pcampus.edu.np> 
        Kushal Paudel <077bct039.kushal@pcampus.edu.np>",
        )
        .about(
            "About: 
        A tool not to just search files but much more.",
        )
        .setting(ArgRequiredElseHelp)
        .args(&[
            Arg::new("STRINGTOFIND"),
            Arg::new("FILENAME")
                .long("file")
                .short('f')
                .takes_value(true)
                .required(false)
                .help("Single or multiple file names"),
            Arg::new("RECURSIVE")
                .long("recursive")
                .short('r')
                .help("Search recursively across dir"),
            Arg::new("IGNORE")
                .long("ignore")
                .short('i')
                .help("ignore case sensetive")
                .takes_value(false),
            Arg::new("HAS_REGEX")
                .long("regex-pattern")
                .short('p')
                .help("find the regex pattern")
                .takes_value(false),
            Arg::new("CURRENT_DIRECTORY")
                .long("current-directory")
                .short('c')
                .help("Search in current directory"),
            Arg::new("WORD")
				.long("word")
				.short('w')
				.help("Search a exact word in file"),
            Arg::new("PREPROCESSOR")
                .long("pre-processor")
                .help("Serialize the file into a trie to search for better efficency"),
        ])
        .get_matches()
}
