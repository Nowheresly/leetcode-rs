
pub struct Solution {}

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut min_penalty = 0;
        let mut cur_penalty = 0;
        let mut earliest_hour = 0;
        let ch = customers.as_bytes();

        for i in 0..customers.len() {
            if ch[i] == b'Y' {
                cur_penalty -= 1;
            } else {
                cur_penalty += 1;
            }
            if cur_penalty < min_penalty {
                min_penalty = cur_penalty;
                earliest_hour = i as i32 + 1;
            }
        }
        earliest_hour
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::best_closing_time(String::from("YYNY")));
    }
}
