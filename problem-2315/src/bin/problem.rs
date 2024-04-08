// This file executes the solution for problem_2315

use problem_2315::problem::Solution;

pub fn main() {
    let result = Solution::count_asterisks("l|*e*et|c**o|*de|".to_string());
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::count_asterisks("l|*e*et|c**o|*de|".to_string()),
        2
    );
}
