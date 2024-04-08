// This file executes the solution for problem_1424

use problem_1424::problem::Solution;

pub fn main() {
    let r = Solution::find_diagonal_order(nums);
    let result = Solution::find_diagonal_order(
        [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec(),
    );
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::find_diagonal_order(
            [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec(),
        ),
        Vec::from([1, 4, 2, 7, 5, 3, 8, 6, 9])
    );
}
