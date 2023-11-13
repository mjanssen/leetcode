pub struct Solution {}

impl Solution {
    pub fn parse_num(n: i32) -> String {
        let res = match n {
            x if x % 3 == 0 && x % 5 == 0 => "FizzBuzz".to_string(),
            x if x % 3 == 0 => "Fizz".to_string(),
            x if x % 5 == 0 => "Buzz".to_string(),
            _ => n.to_string(),
        };

        res
    }

    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let list: Vec<String> = (1..=n).map(|i| Solution::parse_num(i)).collect();
        list
    }
}
