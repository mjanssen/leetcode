// problem_935 - https://leetcode.com/problems/knight-dialer/description/

use std::collections::HashMap;

static MOD: i32 = 1_000_000_007;

pub struct Solution {}

impl Solution {
    fn get_matrix() -> HashMap<i32, Vec<i32>> {
        let mut matrix = HashMap::new();

        matrix.insert(0, vec![4, 6]);
        matrix.insert(1, vec![6, 8]);
        matrix.insert(2, vec![7, 9]);
        matrix.insert(3, vec![4, 8]);
        matrix.insert(4, vec![0, 3, 9]);
        matrix.insert(5, vec![]);
        matrix.insert(6, vec![0, 1, 7]);
        matrix.insert(7, vec![2, 6]);
        matrix.insert(8, vec![1, 3]);
        matrix.insert(9, vec![2, 4]);

        matrix
    }

    fn pad_jump(
        remain: i32,
        square: i32,
        matrix: &HashMap<i32, Vec<i32>>,
        memoized: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if remain == 0 {
            return 1;
        }

        if memoized[remain as usize][square as usize] != 0 {
            return memoized[remain as usize][square as usize];
        }

        let mut ans = 0;

        for next_square in matrix.get(&square).unwrap() {
            ans = (ans + Solution::pad_jump(remain - 1, *next_square, matrix, memoized)) % MOD;
        }

        // Store answer for memoization
        match memoized.get_mut(remain as usize) {
            Some(list) => {
                list[square as usize] = ans;
            }
            _ => (),
        };

        ans
    }

    pub fn knight_dialer(n: i32) -> i32 {
        let matrix = Solution::get_matrix();
        let mut memoized: Vec<Vec<i32>> = vec![vec![0; 10]; n as usize + 1];

        let mut ans = 0;
        for sq in 0..=9 {
            ans = (ans + Solution::pad_jump(n - 1, sq, &matrix, &mut memoized)) % MOD;
        }

        ans
    }
}
