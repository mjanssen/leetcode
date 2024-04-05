// This file executes the solution for task_2391

use task_2391::task::Solution;

pub fn main() {
    let result = Solution::garbage_collection(
        [
            "G".to_string(),
            "P".to_string(),
            "GP".to_string(),
            "GG".to_string(),
        ]
        .to_vec(),
        [2, 4, 3].to_vec(),
    );
}

#[test]
fn test_solution() {
    assert_eq!(
        Solution::garbage_collection(
            [
                "G".to_string(),
                "P".to_string(),
                "GP".to_string(),
                "GG".to_string(),
            ]
            .to_vec(),
            [2, 4, 3].to_vec(),
        ),
        21
    );
}
