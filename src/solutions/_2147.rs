pub struct Solution;

static MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let total_count = corridor
            .chars()
            .filter(|b| b == &'S')
            .collect::<Vec<char>>()
            .len();

        if total_count < 2 {
            return 0;
        }

        if total_count == 2 {
            return 1;
        }

        if total_count % 2 != 0 {
            return 0;
        }

        corridor
            .chars()
            .enumerate()
            .fold(
                (0, 0, 0),
                |(mut ans, mut seats, mut previous_seat_index), (index, current_char)| {
                    match current_char {
                        'S' => {
                            // If seat is found, increment seat
                            seats += 1;
                            if seats == 2 {
                                previous_seat_index = index;
                                if ans == 0 {
                                    ans = 1;
                                }
                            }

                            // If there are > 2 seats, reset seats to one and multiply by previous
                            // possibilities (by using previous seat index)
                            if seats == 3 {
                                seats = 1;
                                ans = (ans * (index - previous_seat_index) as i64) % MOD;
                            }
                        }
                        _ => (),
                    }

                    (ans, seats, previous_seat_index)
                },
            )
            .0 as i32
    }
}
