// This file executes the solution for problem_1630

use problem_1630::problem::Solution;

pub fn main() {
    let result = Solution::check_arithmetic_subarrays(
        [4, 6, 5, 9, 3, 7].to_vec(),
        [0, 0, 2].to_vec(),
        [2, 3, 5].to_vec(),
    );
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::check_arithmetic_subarrays(
            [4, 6, 5, 9, 3, 7].to_vec(),
            [0, 0, 2].to_vec(),
            [2, 3, 5].to_vec(),
        ),
        [true, false, true].to_vec()
    );
}
