// This file executes the solution for problem_1980

use problem_1980::problem::Solution;

pub fn main() {
    let result =
        Solution::find_different_binary_string(["01".to_string(), "10".to_string()].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::find_different_binary_string(["01".to_string(), "10".to_string()].to_vec()),
        "11".to_string()
    );
}
