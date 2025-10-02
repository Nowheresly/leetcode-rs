
pub struct Solution {}

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut empty = num_bottles;
        let mut drank = num_bottles;
        let mut num_exchange = num_exchange;
        while empty >= num_exchange {
            empty -= num_exchange;
            drank += 1;
            empty += 1;
            num_exchange += 1;
        }
        drank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(15, Solution::max_bottles_drunk(13, 6));
        assert_eq!(13, Solution::max_bottles_drunk(10, 3));
    }
}
