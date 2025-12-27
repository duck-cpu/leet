//Valid Anagram O(n)

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut freq = HashMap::new();

        for i in s.chars() {
            *freq.entry(i).or_insert(0) += 1;
            }

        for i in t.chars() {
            match freq.get_mut(&i) {
                Some(count) => {
                    *count -= 1;
                    if *count < 0 {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
}
