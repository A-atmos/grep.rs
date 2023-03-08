use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CharNode {
    end : bool,
    pub next : HashMap<char, CharNode>,
}

impl CharNode {
    pub fn new() -> Self {
        CharNode {
            next : HashMap::new(),
            end : false,
        }
    }

    pub fn insert(&mut self, string : &str) {
        let first_char = string.chars().next();
        if let Some(character) = &first_char {
            if self.next.contains_key(character) {
                self.next.get_mut(character).unwrap().insert(&string[1..]);
            } else {
                let mut next_charnode = CharNode::new();
                next_charnode.insert(&string[1..]);
                self.next.insert(*character, next_charnode);
            }
        } else {
            self.end = true;
        }
    }

    pub fn search( &mut self, string: &str)->bool{
        let first_char = string.chars().next();
        if let Some(character) = &first_char {
            if self.next.contains_key(character){ 
                return self.next.get_mut(character).unwrap().search(&string[1..]);
            }
            else{
                return false;
            }
        }
        else
        {
            return self.end;
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
        for _ in "I am a good girl Iva".split(' ').map(|word| word_searcher.insert(word.trim())) {};

        assert_eq!(false,word_searcher.search("Girl") );
        assert_eq!(true,word_searcher.search("girl"));
    }
}