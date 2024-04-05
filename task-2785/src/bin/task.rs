// This file executes the solution for task_2785

use task_2785::task::Solution;

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
