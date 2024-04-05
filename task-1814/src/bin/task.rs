// This file executes the solution for task_1814

use task_1814::task::Solution;

pub fn main() {
    let result = Solution::count_nice_pairs([42, 11, 1, 97].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(Solution::count_nice_pairs([42, 11, 1, 97].to_vec()), 2);
}
