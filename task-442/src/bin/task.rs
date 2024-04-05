// This file executes the solution for task_442

use task_442::task::Solution;

pub fn main() {
    let result = Solution::find_duplicates([4, 3, 2, 7, 8, 2, 3, 1].to_vec());
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::find_duplicates([4, 3, 2, 7, 8, 2, 3, 1].to_vec()),
        [3, 2].to_vec()
    );
}
