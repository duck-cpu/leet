// Find The Difference O(n)

use std::collections::HashMap;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut freq: HashMap<char,i32> = HashMap::new();
        
        for c in s.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            match freq.get_mut(&c) {
                Some(count) if *count > 0 => {
                    *count -= 1;
                }
                _ => {
                    return c;
                }
            }
        }
        unreachable!()
    }
}
