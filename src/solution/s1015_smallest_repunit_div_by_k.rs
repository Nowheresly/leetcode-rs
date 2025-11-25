use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k == 1 {
            return 1;
        }
        let mut remain = 1;
        let mut count = 1;
        let mut set = HashSet::new();
        while remain != 0 {
            let tmp = (((remain * 10) % k) + 1) % k;
            if set.contains(&tmp) {
                return -1;
            }
            remain = tmp;
            set.insert(remain);
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::smallest_repunit_div_by_k(1));
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(2));
        assert_eq!(3, Solution::smallest_repunit_div_by_k(3));

    }
}
