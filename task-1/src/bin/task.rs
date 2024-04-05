// This file executes the solution for task_1

use task_1::task::Solution;

pub fn main() {
    let result = Solution::two_sum(Vec::from([2, 7, 11, 15]), 9);
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::two_sum(Vec::from([2, 7, 11, 15]), 9),
        Vec::from([0, 1])
    );
}
