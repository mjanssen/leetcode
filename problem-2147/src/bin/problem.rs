// This file executes the solution for problem_2147

use problem_2147::problem::Solution;

pub fn main() {
    let result = Solution::number_of_ways("SSPPSPS".to_string());
}

#[test]
fn test_solution() {
    assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
}
