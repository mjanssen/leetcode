// task_1727 - https://leetcode.com/problems/largest-submatrix-with-rearrangements/description/

pub struct Solution {}

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        matrix
            .iter()
            // Keep track of consecutive count (vertically) in sepperate array
            .fold((0, vec![0; matrix[0].len()]), |(mut ans, mut cons), row| {
                for i in 0..row.len() {
                    // Keep track of how many 1's are seen on i
                    match row[i] {
                        1 => cons[i] += 1,
                        // Streak is reset when a 0 is encounterd
                        _ => cons[i] = 0,
                    };
                }

                let mut copy = cons.clone();

                copy.sort_unstable();
                copy.reverse();

                for i in 0..copy.len() {
                    ans = std::cmp::max(ans, copy[i] * (i + 1) as i32);
                }

                (ans, cons)
            })
            .0
    }
}
