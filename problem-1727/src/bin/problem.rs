// This file executes the solution for problem_1727

use problem_1727::problem::Solution;

pub fn main() {
    let result = Solution::largest_submatrix(
        [[0, 0, 1].to_vec(), [1, 1, 1].to_vec(), [1, 0, 1].to_vec()].to_vec(),
    );
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::largest_submatrix(
            [[0, 0, 1].to_vec(), [1, 1, 1].to_vec(), [1, 0, 1].to_vec()].to_vec(),
        ),
        4
    );
}
