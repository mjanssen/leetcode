// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/description/

pub struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let amount: usize = nums.len();
        (0..(amount / 2))
            .map(|v| nums[v] + nums[(amount - 1) - v])
            .max()
            .unwrap()
    }
}
