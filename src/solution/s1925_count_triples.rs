
pub struct Solution {}
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut set = std::collections::HashSet::new();
        for i in 1..=n {
            set.insert(i*i);
        }
        let mut ans = 0;
        for a in 1..=n {
            for b in a..=n {
                if set.contains(&(b*b+a*a)) {
                    ans += 2;
                }
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
        assert_eq!(2, Solution::count_triples(5));
        assert_eq!(4, Solution::count_triples(10));
    }
}
