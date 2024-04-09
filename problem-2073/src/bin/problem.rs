// This file executes the solution for problem_2073

use problem_2073::problem::Solution;

pub fn main() {
    let result = Solution::time_required_to_buy([2, 3, 2].to_vec(), 2);
    println!("problem_2073: {:?}", result);
}

#[test]
fn test_solution() {
    assert_eq!(Solution::time_required_to_buy([2, 3, 2].to_vec(), 2), 6);
}

#[test]
fn test_solution_2() {
    assert_eq!(
        Solution::time_required_to_buy([84, 49, 5, 24, 70, 77, 87, 8].to_vec(), 3),
        154
    );
}
