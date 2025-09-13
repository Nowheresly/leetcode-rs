
pub struct Solution {}

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut freq = vec![0; 26];
        let mut maxc = 0;
        let mut maxv = 0;
        for c in s.chars() {
            freq[c as usize - 'a' as usize] += 1;
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                maxv = maxv.max(freq[c as usize - 'a' as usize]);
            } else {
                maxc = maxc.max(freq[c as usize - 'a' as usize]);
            }
        }
        maxc + maxv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::max_freq_sum(String::from("successes")));
        assert_eq!(3, Solution::max_freq_sum(String::from("aeiaeia")));

    }
}
