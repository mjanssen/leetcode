// task_1814 - https://leetcode.com/problems/count-nice-pairs-in-an-array/

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    fn reverse_num(mut num: i32) -> i64 {
        if num < 10 {
            return num.into();
        }

        let radix = 10;
        let mut reversed = 0;

        while num != 0 {
            reversed = reversed * radix + num % radix;
            num /= radix;
        }

        reversed.into()
    }

    pub fn count_nice_pairs(nums: Vec<i32>) -> i64 {
        let mut calculated_keys: HashMap<i64, i64> = HashMap::new();

        for num in nums.iter() {
            let calculated = *num as i64 - Solution::reverse_num(*num);

            match calculated_keys.get(&calculated) {
                Some(count) => calculated_keys.insert(calculated, count + 1),
                _ => calculated_keys.insert(calculated, 1),
            };
        }

        calculated_keys.iter().fold(0i64, |mut st, (_, value)| {
            let i: i64 = (((value - 1) * value) / 2) as i64;
            st += i;
            st
        }) % 1_000_000_007
    }
}
