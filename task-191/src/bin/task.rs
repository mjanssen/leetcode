// This file executes the solution for task_191

use task_191::task::Solution;

pub fn main() {
    let result = Solution::hamming_weight(11);
}

#[test]
fn test_solution() {
    assert_eq!(Solution::hamming_weight(11), 3);
}
