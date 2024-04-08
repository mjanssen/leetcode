// This file executes the solution for problem_935

use problem_935::problem::Solution;

pub fn main() {
    let result = Solution::knight_dialer(1);
}

#[test]
fn test_solution() {
    assert_eq!(Solution::knight_dialer(1), 10);
}
