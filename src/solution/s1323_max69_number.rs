
pub struct Solution {}

impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut ch = num.to_string().chars().collect::<Vec<char>>();
        for i in 0..ch.len() {
            let c = ch[i];
            if c == '6' {
                ch[i] = '9';
                break;
            }
        }
        ch.iter().collect::<String>().parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        //assert_eq!(3, Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]));
        assert_eq!(9969, Solution::maximum69_number(9669));
        assert_eq!(9999, Solution::maximum69_number(9996));
        assert_eq!(9999, Solution::maximum69_number(9999));
    }
}
