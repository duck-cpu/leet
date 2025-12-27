//Two Sum O(n)

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut known = HashMap::new();
        let mut needed: i32;
        let mut ans: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            needed = target - nums[i];

            if known.contains_key(&needed) {
                ans.push(i as i32);
                ans.push(known[&needed] as i32);
                return ans;
            }
            known.insert(nums[i], i);
        }
        ans
    }
}

