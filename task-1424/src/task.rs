// task_1424 - https://leetcode.com/problems/diagonal-traverse-ii/description/

pub struct Solution {}

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<(i32, i32, i32)> = Vec::new();

        for (row_index, row) in nums.into_iter().enumerate() {
            for (i, num) in row.into_iter().enumerate() {
                let index: i32 = row_index as i32 + i as i32;
                result.push((index, i as i32, num));
            }
        }

        result.sort();
        result
            .into_iter()
            .map(|(_, _, num)| num)
            .collect::<Vec<i32>>()
    }
}
