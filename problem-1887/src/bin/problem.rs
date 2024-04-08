// This file executes the solution for problem_1887

use problem_1887::problem::Solution;

pub fn main() {
    let result = Solution::reduction_operations([5, 1, 3].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(Solution::reduction_operations([5, 1, 3].to_vec()), 3);
}
