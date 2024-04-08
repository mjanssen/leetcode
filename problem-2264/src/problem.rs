// problem_2264 - https://leetcode.com/problems/largest-3-same-digit-number-in-string/description/

pub struct Solution {}

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        if num.len() < 3 {
            return "".to_string();
        }

        let mut current: Option<u32> = None;

        for i in 0..num.len() {
            let start = if i > 0 { i - 1 } else { 0 };
            let end = if i < num.len() { i + 1 } else { i };
            let range = num.get(start..=end).unwrap_or("").as_bytes();
            if range.len() == 3 && range.get(0) == range.get(1) && range.get(1) == range.get(2) {
                if current.is_none()
                    || (current.unwrap()
                        < (*range.get(0).unwrap() as char).to_digit(10).unwrap_or(0))
                {
                    current = Some((*range.get(0).unwrap() as char).to_digit(10).unwrap_or(0));
                    if current.unwrap() == 9 {
                        // 9 is the highest number
                        break;
                    }
                }
            }
        }

        if current.is_none() {
            return "".to_string();
        }

        format!("{}", current.unwrap().to_string().repeat(3))
    }
}
