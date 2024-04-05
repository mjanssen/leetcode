// task_1846 - https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/description/

pub struct Solution {}

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort();
        arr.iter().fold(0, |sum, val| {
            if val > &sum {
                return sum + 1;
            };
            sum
        })
    }
}
