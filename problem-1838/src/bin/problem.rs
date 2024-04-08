// This file executes the solution for problem_1838

use problem_1838::problem::Solution;

pub fn main() {
    let result = Solution::max_frequency([1, 2, 4].to_vec(), 5);
}

#[test]
fn test_solution() {
    assert_eq!(Solution::max_frequency([1, 2, 4].to_vec(), 5), 3);
}
