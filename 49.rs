//Group Anagrams O(n * k)

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans: HashMap<[u16; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut freq = [0u16; 26];
            for c in s.chars() {
                freq[(c as u8 - b'a') as usize] += 1;
            }
            ans.entry(freq).or_insert_with(Vec::new).push(s);
        }
        ans.into_values().collect()
    }
}
