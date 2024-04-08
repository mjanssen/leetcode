// This file executes the solution for problem_191

use problem_191::problem::Solution;

pub fn main() {
    let result = Solution::hamming_weight(11);
}

#[test]
fn test_solution() {
    assert_eq!(Solution::hamming_weight(11), 3);
}
