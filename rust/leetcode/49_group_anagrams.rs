use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hmap: HashMap<[usize; 26], Vec<String>> = HashMap::new();

        for s in strs.iter() {
            let mut key: [usize; 26] = [0; 26];

            for c in s.as_bytes() {
                key[(c - b'a') as usize] += 1;
            }

            hmap.entry(key).or_default().push(s.to_owned());
        }

        let mut output: Vec<Vec<String>> = vec![];
        for v in hmap.values() {
            output.push(v.to_owned());
        }

        return output;
    }
}
