// This file executes the solution for task_70

use task_70::task::Solution;

pub fn main() {
    let result = Solution::climb_stairs(2);
}

#[test]
fn test_solution() {
    assert_eq!(Solution::climb_stairs(2), 2);
}
