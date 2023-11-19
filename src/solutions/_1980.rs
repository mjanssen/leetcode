pub struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result: String = nums
            .iter()
            .enumerate()
            .map(|(i, v)| match v.as_bytes()[i] {
                48 => "1".to_string(),
                49 => "0".to_string(),
                _ => panic!("Non binary value found"),
            })
            .collect::<Vec<String>>()
            .join("");

        if result.len() < nums[0].len() {
            for _i in result.len()..nums[0].len() {
                result.push('0');
            }
        }

        result
    }
}
