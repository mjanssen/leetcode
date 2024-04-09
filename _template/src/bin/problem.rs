// This file executes the solution for {{crate_name}}

use {{crate_name}}::problem::Solution;

pub fn main() {
    let result = Solution::{{fn_name}}();
    println!("{{crate_name}}: {:?}", result);
}

#[test]
fn test_solution() {
    assert_eq!(Solution::{{fn_name}}(), {});
}
