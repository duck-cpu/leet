// Ransom Note O(n)

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut freq = HashMap::new();

        for c in magazine.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }

        for c in ransom_note.chars() {
            match freq.get_mut(&c) {
                Some(count) if *count > 0 => *count -= 1,
                _ => return false,
            }
        }
        true
    }
}
