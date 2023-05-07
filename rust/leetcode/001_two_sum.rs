
// Brute force
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for idx1 in 0..(nums.len()-1) {
            for idx2 in (idx1+1)..nums.len() {
                if nums.get(idx1).unwrap() + nums.get(idx2).unwrap() == target {
                    return vec![idx1 as i32, idx2 as i32];
                }
            }
        }
        
        return vec![0,0];
    }
}

// O(n)
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut value_index_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (idx, &nb) in nums.iter().enumerate() {
            let remaining = target - nb;
            match value_index_map.get(&remaining) {
                Some(&idx2) => return vec![idx2, idx as i32],
                _ => {
                    value_index_map.insert(nb, idx as i32);
                },
            }
        }

        unreachable!();
        // return vec![-1, -1];
    }
}