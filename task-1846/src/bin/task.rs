// This file executes the solution for task_1846

use task_1846::task::Solution;

pub fn main() {
    let result =
        Solution::maximum_element_after_decrementing_and_rearranging([2, 2, 1, 2, 1].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::maximum_element_after_decrementing_and_rearranging([2, 2, 1, 2, 1].to_vec()),
        2
    );
}
