use std::collections::{BinaryHeap, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn rearrange_string(s: String, k: i32) -> String {
        if k < 2 {
            return s;
        }
        let ch = s.chars().collect::<Vec<char>>();
        let mut freq = vec![0; 26];
        for c in ch {
            freq[c as usize - 'a' as usize] += 1;
        }
        let mut busy = BinaryHeap::new();
        for i in 0..26 {
            if freq[i] > 0 {
                busy.push((freq[i], (i as u8 + 'a' as u8) as char));
            }
        }
        let mut ans = String::with_capacity(s.len());
        let mut wait = VecDeque::new();

        while let Some((f, c)) = busy.pop() {

            ans.push(c);

            wait.push_back((f - 1, c));

            if wait.len() < k as usize {
                continue;
            }

            let prev = wait.pop_front().unwrap();
            if prev.0 > 0 {
                busy.push(prev);
            }
        }
        while wait.is_empty() == false {
            let data = wait.pop_front().unwrap();
            if data.0 > 0 {
                return String::new();
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            String::from("cbacba"),
            Solution::rearrange_string(String::from("aabbcc"), 3));

    }
}
