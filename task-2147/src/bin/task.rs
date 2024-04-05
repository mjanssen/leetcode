// This file executes the solution for task_2147

use task_2147::task::Solution;

pub fn main() {
    let result = Solution::number_of_ways("SSPPSPS".to_string());
}

#[test]
fn test_solution() {
    assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
}
