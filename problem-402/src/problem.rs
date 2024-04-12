// problem_402 - https://leetcode.com/problems/remove-k-digits/description/

use std::{char::from_digit, mem::replace};

pub struct Solution {}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        if num.len() == k as usize {
            return "0".to_string();
        }

        let mut count = 0;
        let mut digits: Vec<u32> = Vec::new();

        for (i, n) in num.chars().filter_map(|c| c.to_digit(10)).enumerate() {
            if i == 0 {
                digits.push(n);
                continue;
            }

            if count.eq(&k) {
                digits.push(n);
                continue;
            }

            if let Some(d) = digits.last() {
                if &n < d {
                    while &n < digits.last().unwrap_or(&0) {
                        if count.eq(&k) {
                            break;
                        }

                        digits.pop();
                        count += 1;
                    }
                    digits.push(n);
                } else {
                    digits.push(n);
                }
            } else {
                digits.push(n);
            }
        }

        if count < k {
            digits.truncate(digits.len().saturating_sub((k - count) as usize));
        }

        let res = digits
            .into_iter()
            .map(|c| from_digit(c, 10).unwrap())
            .collect::<String>()
            .trim_start_matches('0')
            .to_string();

        if res.eq(&"") {
            return "0".to_string();
        }

        res
    }
}
