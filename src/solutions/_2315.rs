pub struct Solution {}

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        if s.contains("*") == false {
            return 0;
        }

        let mut asterisks_count = 0;

        for (i, s) in s.split("|").enumerate() {
            if i % 2 == 0 {
                asterisks_count += s.matches("*").count();
            }
        }

        asterisks_count as i32
    }
}
