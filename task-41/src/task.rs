// task_41 - https://leetcode.com/problems/first-missing-positive/description/

use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let sorted_positive = nums.iter().collect::<BTreeSet<_>>();

        if let Some(last) = nums.last() {
            if last < &1 {
                return 1;
            }

            for num in 1..=*last {
                if sorted_positive.contains(&&num) == false {
                    return num;
                }
            }

            return last + 1;
        }

        1
    }
}
