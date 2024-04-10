// problem_2073 - https://leetcode.com/problems/time-needed-to-buy-tickets/description/

pub struct Solution {}

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let find = tickets[k as usize];

        tickets
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                if i <= k as usize {
                    return x.min(find);
                }

                x.min(find - 1)
            })
            .sum()
    }
}
