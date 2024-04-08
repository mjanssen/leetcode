// This file executes the solution for problem_1

use problem_1::problem::Solution;

pub fn main() {
    let result = Solution::two_sum(Vec::from([2, 7, 11, 15]), 9);
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::two_sum(Vec::from([2, 7, 11, 15]), 9),
        Vec::from([0, 1])
    );
}
