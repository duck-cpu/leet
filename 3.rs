//Longest Substring Without Repeating Characters O(n)..ish
//VecDeque not necessarily ideal because of lookup time

use std::collections::VecDeque;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut window = VecDeque::new();
        let mut maxlen = 0;

        for c in s.chars() {
            while window.contains(&c) {
                window.pop_back();
            }
            window.push_front(c);
            maxlen = maxlen.max(window.len());
        }
        
        maxlen as i32
    }
}
