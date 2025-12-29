//Longest Palindrome O(n)

use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut freq = HashMap::new();
        let mut ans = 0;
        let mut odds: bool = true;

        for c in s.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }

        for &c in freq.values() {
            ans += (c / 2) * 2;
            if c % 2 == 1 && odds == true {
                ans += 1;
                odds = false;
            }
        }
        ans
    }
}
