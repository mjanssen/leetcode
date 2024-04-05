// task_1561 - https://leetcode.com/problems/maximum-number-of-coins-you-can-get/description/

pub struct Solution {}

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();

        let length = piles.len();

        (0..length / 3).fold(0, |st, i| (st + piles[length - (i * 2) - 2]))
    }
}
