pub struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        // Sort vec
        nums.sort_unstable();

        let (mut left, mut ans, mut curr) = (0, 0, 0);

        for i in 0..nums.len() {
            let target = nums[i];
            curr += target;

            while (i - left + 1) as i32 * target - curr > k {
                curr -= nums[left];
                left += 1;
            }

            // Update answer based on window size
            ans = std::cmp::max(ans, i - left + 1);
        }

        ans as i32
    }
}
