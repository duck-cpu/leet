//Contains Duplicate II O(n)

use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut window: HashSet<i32> = HashSet::new();
        let mut L = 0;

        for R in 0..nums.len(){
            if R - L > k as usize {
                window.remove(&nums[L]);
                L += 1;
            }
            if window.contains(&nums[R]) {
                return true;
            }
            window.insert(nums[R]);
        }
        false
    }
}
