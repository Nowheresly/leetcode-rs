
pub struct Solution {}

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..n {
            let a = i;
            let b = n - i;
            if contains_zero(a) || contains_zero(b) {
                continue;
            }
            return vec![a, b];
        }
        vec![-1,-1]
    }
}

fn contains_zero(a: i32) -> bool {
    if a == 0 {
        return true;
    }
    let mut b = a;
    while b != 0 {
        if b % 10 == 0 {
            return true;
        }
        b /= 10;
    }
    false   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = Solution::get_no_zero_integers(2);
        assert_eq!(2, ret.len());
        assert_eq!(2, ret.into_iter().sum::<i32>());
    }
}
