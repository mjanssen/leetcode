// This file executes the solution for problem_1814

use problem_1814::problem::Solution;

pub fn main() {
    let result = Solution::count_nice_pairs([42, 11, 1, 97].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(Solution::count_nice_pairs([42, 11, 1, 97].to_vec()), 2);
}
