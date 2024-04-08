// problem_70 - https://leetcode.com/problems/climbing-stairs/description/

pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        (2..=n)
            .into_iter()
            .fold((1, 1), |(mut step_1, mut step_2), _| {
                let ans = step_1 + step_2;
                step_1 = step_2;
                step_2 = ans;

                (step_1, step_2)
            })
            .1
    }
}
