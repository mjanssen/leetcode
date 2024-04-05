// This file executes the solution for task_1685

use task_1685::task::Solution;

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
