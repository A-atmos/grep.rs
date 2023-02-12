
#[cfg(test)]

mod test {
    use grep_rs::matcher;


    #[test]
    fn check_for_regex_in_a_str() {

        assert_eq!(true, matcher::is_match_regex(String::from("This line consists of new."),String::from("*new*")));
        assert_eq!(true, matcher::is_match_regex(String::from("This is a test file."),String::from("*t??t*")));
        assert_eq!(true, matcher::is_match_regex(String::from("Here is a new sentence."),String::from("*s*c*")));
        assert_eq!(true, matcher::is_match_regex(String::from("This line consists of new."),String::from("*l*e ?o*")));//line co
        assert_eq!(false, matcher::is_match_regex(String::from("This line consists of new."),String::from("*T*ec*")));//This linec -> false

    }
    
}