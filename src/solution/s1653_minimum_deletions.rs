

pub struct Solution {
}

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let n = s.len();
        let mut min_deletions = 0;
        let mut b_count = 0;
        let ch:Vec<char> = s.chars().collect();

        for i in 0..n {
            if ch[i] == 'b' {
                b_count += 1;
            } else {
                min_deletions = b_count.min(min_deletions + 1);
            }
        }
        min_deletions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::minimum_deletions(String::from("aababbab")));

    }
}
