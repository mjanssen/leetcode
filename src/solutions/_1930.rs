use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut pairs: HashSet<String> = HashSet::new();
        let mut starts: HashSet<char> = HashSet::new();

        for (i, c) in s.chars().enumerate() {
            if starts.contains(&c) {
                continue;
            }

            let rest = &s[i + 1..];

            if let Some(index) = rest.rfind(c) {
                starts.insert(c);

                let mut r: Vec<char> = rest[..index].chars().collect();
                let mut uniques = HashSet::new();

                r.retain(|v| uniques.insert(v.clone()));

                for inner in r {
                    let palindrome = format!("{}{}{}", c, inner, c);
                    if pairs.get(&palindrome).is_none() {
                        pairs.insert(palindrome);
                    }
                }
            }
        }

        pairs.len() as i32
    }
}
