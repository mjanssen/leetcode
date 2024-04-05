// This file executes the solution for task_1561

use task_1561::task::Solution;

pub fn main() {
    let result = Solution::max_coins([2, 4, 1, 2, 7, 8].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(Solution::max_coins([2, 4, 1, 2, 7, 8].to_vec()), 9);
}
