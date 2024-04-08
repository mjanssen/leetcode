// This file executes the solution for problem_1685

use problem_1685::problem::Solution;

pub fn main() {
    let result = Solution::get_sum_absolute_differences([2, 3, 5].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::get_sum_absolute_differences([2, 3, 5].to_vec()),
        [4, 3, 5].to_vec()
    );
}
