// This file executes the solution for problem_2785

use problem_2785::problem::Solution;

pub fn main() {
    let result = Solution::sort_vowels("lEetcOde".to_string());
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::sort_vowels("lEetcOde".to_string()),
        "lEOtcede".to_string()
    );
}
