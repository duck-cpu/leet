// Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold O(n*k)

use std::collections::VecDeque;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut window: VecDeque<i32> = VecDeque::new();
        let mut count = 0;
        
        for R in 0..arr.len() {
            window.push_back(arr[R]);
            if window.len() > k as usize {
                window.pop_front();
            }
            if window.len() == k as usize {

                let sum: i32 = window.iter().sum();
                if (sum / window.len() as i32 >= threshold) {
                    count += 1;
                }
            }
        }
        count
    }
}
