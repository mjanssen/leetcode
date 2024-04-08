// This file executes the solution for problem_2264

use problem_2264::problem::Solution;

pub fn main() {
    let result = Solution::largest_good_integer("6777133339".to_string());
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::largest_good_integer("6777133339".to_string()),
        "777".to_string()
    );
}
