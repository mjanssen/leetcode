// This file executes the solution for task_2315

use task_2315::task::Solution;

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
