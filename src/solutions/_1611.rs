// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/description/

pub struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let binary = format!("{n:b}");
        let chars = binary.chars();
    
        for (i, char) in chars.enumerate() {
            println!("{char}");
        }

        todo!()
    }
}
