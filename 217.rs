//Contains Duplicate O(n)

use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut freq = HashMap::new();

        for i in 0..nums.len() {
            if freq.contains_key(&nums[i]) {
                return true;
            }
            freq.entry(nums[i]).or_insert(i);
        }
        false
    }
}
