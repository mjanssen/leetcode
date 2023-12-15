// https://leetcode.com/problems/two-sum/submissions/1120183124/description

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut memoized: HashMap<_, _> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(index) = memoized.get(&(target - num)) {
                return vec![*index, i as i32];
            }

            memoized.insert(num, i as i32);
        }

        return Vec::new();
    }
}
