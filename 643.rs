//Maximum Average Subarray 1 O(n)

use std::collections::VecDeque;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum: i32 = 0;
        let mut window: VecDeque<i32> = VecDeque::new();
        let mut max_sum: f64 = f64::NEG_INFINITY;

        for n in nums {
            window.push_front(n);
            sum += n;
            
            if window.len() > k as usize{
                if let Some(out) = window.back() {
                    sum -= out;
                    window.pop_back();
                }  
            }

            if window.len() == k as usize {
                max_sum = max_sum.max(sum as f64);
            }
        }
        max_sum / k as f64
    }
}
