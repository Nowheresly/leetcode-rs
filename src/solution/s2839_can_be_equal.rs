
pub struct Solution {}

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut freqodd1 = vec![0;26];
        let mut freqodd2 = vec![0;26];
        let mut freqeven1 = vec![0;26];
        let mut freqeven2 = vec![0;26];
        for i in 0..s1.len() {
            if i % 2 == 0 {
                freqeven1[s1.as_bytes()[i] as usize - b'a' as usize] += 1;
                freqeven2[s2.as_bytes()[i] as usize - b'a' as usize] += 1;
            } else {
                freqodd1[s1.as_bytes()[i] as usize - b'a' as usize] += 1;
                freqodd2[s2.as_bytes()[i] as usize - b'a' as usize] += 1;
            }
        }
        for i in 0..26 {
            if freqeven1[i] != freqeven2[i] {
                return false;
            }
            if freqodd1[i] != freqodd2[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::can_be_equal(String::from("abcd"), String::from("cdab")));

    }
}
