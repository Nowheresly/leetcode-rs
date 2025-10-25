
pub struct Solution {
}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut res = 0;
        let mut last_monday = 0;
        let mut count = 0;
        for i in 0..n {
            if i % 7 == 0 {
                last_monday += 1;
                count = last_monday;
            } else {
                count += 1;
            }
            res += count;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(10, Solution::total_money(4));
        assert_eq!(37, Solution::total_money(10));
        assert_eq!(96, Solution::total_money(20));

    }
}
