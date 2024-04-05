// This file executes the solution for task_935

use task_935::task::Solution;

pub fn main() {
    let result = Solution::knight_dialer(1);
}

#[test]
fn test_solution() {
    assert_eq!(Solution::knight_dialer(1), 10);
}
