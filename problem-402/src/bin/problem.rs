// This file executes the solution for problem_402

use problem_402::problem::Solution;

pub fn main() {
    let result = Solution::remove_kdigits("1234567890".to_string(), 9);
    println!("problem_402: {:?}", result);
}

#[test]
fn test_solution_1() {
    assert_eq!(
        Solution::remove_kdigits("1432219".to_string(), 3),
        "1219".to_string()
    );
}

#[test]
fn test_solution_2() {
    assert_eq!(
        Solution::remove_kdigits("10200".to_string(), 1),
        "200".to_string()
    );
}

#[test]
fn test_solution_3() {
    assert_eq!(
        Solution::remove_kdigits("10".to_string(), 2),
        "0".to_string()
    );
}

#[test]
fn test_solution_4() {
    assert_eq!(
        Solution::remove_kdigits("112".to_string(), 1),
        "11".to_string()
    );
}

#[test]
fn test_solution_5() {
    assert_eq!(
        Solution::remove_kdigits("1234567890".to_string(), 9),
        "0".to_string()
    );
}
