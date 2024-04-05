// This file executes the solution for task_41

use task_41::task::Solution;

pub fn main() {
    let result = Solution::first_missing_positive([1, 2, 0].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(Solution::first_missing_positive([1, 2, 0].to_vec()), 3);
}
