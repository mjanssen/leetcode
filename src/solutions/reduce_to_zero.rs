pub struct Solution;

impl Solution {
    pub fn recursive_check(num: i32) -> i32 {
        if num.eq(&0) {
            return 0;
        }

        match num {
            x if x % 2 == 0 => num / 2,
            _ => num - 1,
        }
    }

    pub fn number_of_steps(num: i32) -> i32 {
        if num.eq(&0) {
            return 0;
        }

        let mut value = num.clone();
        let mut amount = 1;

        while Self::recursive_check(value) != 0 {
            value = Self::recursive_check(value);
            amount += 1;
        }

        amount
    }
}
