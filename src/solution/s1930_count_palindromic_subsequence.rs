use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let n = s.len();
        let ch = s.chars().collect::<Vec<char>>();
        let mut map = vec![0; 26];

        for i in 1..n {
            let index = ch[i] as usize - 'a' as usize;
            map[index] = map[index] + 1;
        }
        let mut set = HashSet::new();
        set.insert(ch[0]);

        let mut res = HashSet::new();

        for i in 1..n {
            let c = ch[i];
            map[c as usize - 'a' as usize] -= 1;
            for l in set.iter() {
                if map[*l as usize - 'a' as usize] > 0 {
                    res.insert(format!("{}{}{}", l, c, l));
                }
            }
            set.insert(c);
        }

        res.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::count_palindromic_subsequence("aabca".to_string())
        );
    }
}
