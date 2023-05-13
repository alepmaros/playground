// Given two strings s and t, return true if t is an anagram of s, and false otherwise.

use std::collections::HashMap;

// Second Solution -- prettier
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut sh: HashMap<char, i32> = HashMap::new();

        s.chars().for_each(|c| *sh.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *sh.entry(c).or_insert(0) -= 1);
        return sh.into_values().all(|v| v == 0);

    }
}

// First Solution
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut sh: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            match sh.get(&c) {
                Some(count) => { sh.insert(c, count + 1); },
                None => { sh.insert(c, 1); }
            }
        }

        for c in t.chars() {
            if let Some(v) = sh.get(&c) {
                let new_value = v - 1;
                if new_value < 0 {
                    return false
                }
                sh.insert(c, new_value);
            } else {
                return false
            }
        
        }

        for v in sh.values() {
            if *v > 0 {
                return false;
            }
        }

        return true
    }
}