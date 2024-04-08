// problem_2785 - https://leetcode.com/problems/sort-vowels-in-a-string/description/

const VOWELS: &str = "aeiouAEIOU";

pub struct Solution {}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels: Vec<char> = s.chars().filter(|c| VOWELS.contains(*c)).collect();

        // Order
        vowels.sort();
        vowels.reverse();

        let result: Vec<String> = s
            .chars()
            .map(|v| {
                if VOWELS.contains(v) {
                    if let Some(first) = vowels.pop() {
                        return first.to_string();
                    }
                }

                v.to_string()
            })
            .collect();

        result.join("")
    }
}
