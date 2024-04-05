// This file executes the solution for task_1930

use task_1930::task::Solution;

pub fn main() {
    let result = Solution::count_palindromic_subsequence("aabca".to_string());
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::count_palindromic_subsequence("aabca".to_string()),
        3
    );
}
