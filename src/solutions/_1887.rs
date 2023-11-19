pub struct Solution;

impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.reverse();

        let lowest_number = nums[nums.len() - 1];

        let mut operations: i32 = 0;

        for (i, num) in nums.iter().enumerate() {
            let next = match nums.get(i + 1) {
                Some(n) => n,
                _ => &lowest_number,
            };

            if next < num {
                operations += 1 + i as i32;
            }
        }

        operations
    }
}
