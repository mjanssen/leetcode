// This file executes the solution for problem_70

use problem_70::problem::Solution;

pub fn main() {
    let result = Solution::climb_stairs(2);
}

#[test]
fn test_solution() {
    assert_eq!(Solution::climb_stairs(2), 2);
}
