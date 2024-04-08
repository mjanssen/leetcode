// problem_1980 - https://leetcode.com/problems/find-unique-binary-string/description/

pub struct Solution {}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        nums.iter()
            .enumerate()
            .map(|(i, v)| match v.as_bytes()[i] {
                48 => "1".to_string(),
                49 => "0".to_string(),
                _ => panic!("Non binary value found"),
            })
            .collect::<Vec<String>>()
            .join("")
    }
}
