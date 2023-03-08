use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CharNode {
    end : bool,
    pub next : HashMap<char, CharNode>,
    pub line_no : Vec<usize>,

}

impl CharNode {
    pub fn new() -> Self {
        CharNode {
            next : HashMap::new(),
            end : false,
            line_no : vec![],
        }
    }

    pub fn insert(&mut self, string : &str, _line_no : usize) {
        let first_char = string.chars().next();
        if let Some(character) = &first_char {
            if self.next.contains_key(character) {
                let string_vec = string.chars().collect::<Vec<_>>();
                let _string = string_vec[1..].iter().cloned().collect::<String>();
                self.next.get_mut(character).unwrap().insert(&_string,_line_no);
            } else {
                let mut next_charnode = CharNode::new();
                let string_vec = string.chars().collect::<Vec<_>>();
                let _string = string_vec[1..].iter().cloned().collect::<String>();
                next_charnode.insert(&_string,_line_no);
                self.next.insert(*character, next_charnode);
            }
        } else {
            self.end = true;
            self.line_no.push(_line_no);
        }
    }

    pub fn search( &mut self, string: &str)->(bool,Vec<usize>){
        let first_char = string.chars().next();
        if let Some(character) = &first_char {
            if self.next.contains_key(character){ 
                return self.next.get_mut(character).unwrap().search(&string[1..]);
            }
            else{
                return (false,vec![]);
            }
        }
        else
        {
            return (self.end, self.line_no.clone());
        }
    }


    


}


#[cfg(test)]
mod test {

    // use grep_rs::trie::CharNode;
    use super::*;

    #[test]

    fn trie_search(){
        let mut word_searcher = CharNode::new();
        for _ in "I am a good girl Iva".split(' ').map(|word| word_searcher.insert(word.trim(),0)) {};

        assert_eq!(false,word_searcher.search("Girl").0);
        assert_eq!(true,word_searcher.search("girl").0);
    }
}