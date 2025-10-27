pub struct Solution {}

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut sum = 0;
        let mut prev = 0;
        for str in bank.iter() {
            let count = count1(str);
            if count > 0 {
                sum += count * prev;
                prev = count;
            }
        }
        sum
    }
}

fn count1(str: &str) -> i32 {
    let mut ret = 0;
    for c in str.chars() {
        if c == '1' {
            ret += 1;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::number_of_beams(vec![String::from("011001"),String::from("000000"),String::from("010100"),String::from("001000")]));

    }
}
