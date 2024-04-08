// problem_442 - https://leetcode.com/problems/find-all-duplicates-in-an-array/description/

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .fold(HashMap::new(), |mut hm, num| {
                match hm.get(&num) {
                    Some(count) => {
                        hm.insert(num, count + 1);
                    }
                    None => {
                        hm.insert(num, 1);
                    }
                }

                hm
            })
            .iter()
            .filter_map(|(n, count)| {
                if count > &1 {
                    return Some(**n);
                }

                None
            })
            .collect()
    }
}
