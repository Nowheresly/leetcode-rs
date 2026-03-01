
pub struct Solution {
}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut max = 0;
        for c in n.chars() {
            if c.to_digit(10).unwrap() > max {
                max = c.to_digit(10).unwrap();
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::min_partitions(String::from("27346209830709182346")));
    }
}
