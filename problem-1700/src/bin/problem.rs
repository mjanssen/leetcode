// This file executes the solution for problem_1700

use problem_1700::problem::Solution;

pub fn main() {
    let result = Solution::count_students([1, 1, 1, 0, 0, 1].to_vec(), [1, 0, 0, 0, 1, 2].to_vec());
    println!("problem_1700: {:?}", result);
}

#[test]
fn test_case_1() {
    assert_eq!(
        Solution::count_students([1, 1, 0, 0].to_vec(), [0, 1, 0, 1].to_vec()),
        0
    );
}

#[test]
fn test_case_2() {
    assert_eq!(
        Solution::count_students([1, 1, 1, 0, 0, 1].to_vec(), [1, 0, 0, 0, 1, 2].to_vec()),
        3
    );
}
