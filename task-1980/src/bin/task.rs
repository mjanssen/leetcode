// This file executes the solution for task_1980

use task_1980::task::Solution;

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
