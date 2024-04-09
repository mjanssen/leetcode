// problem_1700 - https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/description/

use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn count_students(students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut studs = VecDeque::from(students);
        let mut sands = VecDeque::from(sandwiches);

        let mut serve_tries = 0;

        while let Some(student) = studs.pop_front() {
            if student.eq(sands.front().unwrap_or(&-1)) {
                serve_tries = 0;
                sands.pop_front();
            } else {
                serve_tries += 1;
                studs.push_back(student);
            }

            if serve_tries.eq(&studs.len()) {
                break;
            }
        }

        studs.len() as i32
    }
}
