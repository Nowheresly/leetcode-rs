pub struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let ch1:Vec<char> = s.chars().collect();
        let n = s.len();
        let mut res1 = 0;
        let mut val = true;
        for i in 0..n {
            if val {
                if ch1[i] == '0' {
                    res1 += 1;
                }
            } else {
                if ch1[i] == '1' {
                    res1 += 1;
                }
            }
            val = !val;
        }

        let mut res0 = 0;
        val = false;
        for i in 0..n {
            if val {
                if ch1[i] == '0' {
                    res0 += 1;
                }
            } else {
                if ch1[i] == '1' {
                    res0 += 1;
                }
            }
            val = !val;
        }
        if res0 < res1 { res0 } else { res1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_operations(String::from("0100")));
        assert_eq!(2, Solution::min_operations(String::from("1111")));
    }
}
