// task_1 - https://leetcode.com/problems/two-sum/description/
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Keep a hashmap that stores the desired numbers with their index
        let mut memoized: HashMap<_, _> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            // Check if the memoized hashmap contains the desired number
            if let Some(index) = memoized.get(&(target - num)) {
                return vec![*index, i as i32];
            }

            // Store the current number in the hashmap with its index
            memoized.insert(num, i as i32);
        }

        return Vec::new();
    }
}
