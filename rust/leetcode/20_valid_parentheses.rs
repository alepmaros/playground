// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let hmap: HashMap<char, char> = HashMap::from([
            ('{', '}'),
            ('(', ')'),
            ('[', ']')
        ]);

        for c in s.chars() {
            match c {
                '{' | '(' | '[' => { stack.push(*hmap.get(&c).unwrap()); },
                '}' | ')' | ']' => { 
                    if let Some(pop_c) = stack.pop() {
                        if pop_c != c {
                            return false
                        }
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }

        return stack.is_empty();
    }
}
