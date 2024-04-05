// task_191 - https://leetcode.com/problems/number-of-1-bits/description/

pub struct Solution {}

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
}
