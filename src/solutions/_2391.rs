pub struct Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let types = ["M", "P", "G"];

        garbage
            .iter()
            .enumerate()
            .fold([(0, 0), (0, 0), (0, 0)], |mut st, (i, el)| {
                for (garbage_index, garbage_type) in types.iter().enumerate() {
                    if el.contains(garbage_type) {
                        st[garbage_index].0 += el.matches(garbage_type).count();
                        st[garbage_index].1 = i;
                    }
                }

                st
            })
            .iter()
            .fold(0, |mut st, (garbage_amount, last_index)| {
                if garbage_amount == &0 {
                    return st;
                }

                st += garbage_amount;
                st += travel[..*last_index as usize].iter().fold(0, |mut t, x| {
                    t += x;
                    t
                }) as usize;

                st
            }) as i32
    }
}
