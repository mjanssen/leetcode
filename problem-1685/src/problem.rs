// problem_1685 - https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/

pub struct Solution {}

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len() as i32;

        nums.iter()
            .enumerate()
            .map(|(i, num)| {
                let lower: i32 = nums[..i].iter().sum();
                let higher: i32 = nums[i + 1..].iter().sum();

                (num * i as i32 - lower) + (higher - num * (len - 1 - i as i32))
            })
            .collect::<Vec<i32>>()
    }
}
