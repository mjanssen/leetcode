pub struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        matrix
            .iter()
            .fold((0, vec![0; matrix[0].len()]), |(mut ans, mut cons), row| {
                for i in 0..row.len() {
                    match row[i] {
                        1 => cons[i] += 1,
                        _ => cons[i] = 0,
                    };
                }

                let mut copy = cons.clone();
                copy.sort_unstable();
                for i in 0..copy.len() {
                    ans = std::cmp::max(ans, copy[i] * (copy.len() - i) as i32);
                }

                (ans, cons)
            })
            .0
    }
}
