
pub struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let m = strs[0].len();
        let mut dp = vec![1; m];

        for i in 1..m {
            for j in 0..i {
                let mut all_valid = true;
                for s in &strs {
                    if s.as_bytes()[i] < s.as_bytes()[j] {
                        all_valid = false;
                        break;
                    }
                }
                if all_valid {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        m as i32 - dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_deletion_size(vec![String::from("babca"), String::from("bbazb")]));

    }
}
