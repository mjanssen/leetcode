// This file executes the solution for problem_1877

use problem_1877::problem::Solution;

pub fn main() {
    let result = Solution::min_pair_sum([3, 5, 2, 3].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(Solution::min_pair_sum([3, 5, 2, 3].to_vec()), 7);
}
