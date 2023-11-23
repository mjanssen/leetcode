pub struct Solution;

impl Solution {
    fn is_arithmetic(data: &mut Vec<i32>) -> bool {
        data.sort_unstable();
        data.reverse();

        for (i, num) in data.iter().enumerate() {
            if let Some(next) = data.get(i + 1) {
                if next - num != data[1] - data[0] {
                    return false;
                }
            }
        }

        true
    }

    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        l.into_iter()
            .zip(r.into_iter())
            .map(|(il, ir)| nums[il as usize..=ir as usize].to_vec())
            .map(|mut range: Vec<i32>| Solution::is_arithmetic(range.as_mut()))
            .collect::<Vec<bool>>()
    }
}
