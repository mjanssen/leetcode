// This file executes the solution for problem_2788

use problem_2788::problem::Solution;

pub fn main() {
    let result = Solution::split_words_by_separator(
        [
            "one.two.three".to_string(),
            "four.five".to_string(),
            "six".to_string(),
        ]
        .to_vec(),
        '.',
    );
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::split_words_by_separator(
            [
                "one.two.three".to_string(),
                "four.five".to_string(),
                "six".to_string(),
            ]
            .to_vec(),
            '.',
        ),
        [
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string()
        ]
        .to_vec()
    );
}
