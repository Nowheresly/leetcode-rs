pub struct Solution {}

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        for &n in nums.iter() {
            ret += compute(n);
        }
        ret
    }
}

fn compute(n: i32) -> i32 {
    let mut cnt = 2;
    let mut sum = 1 + n;
    for i in 2..n {
        if n % i == 0 {
            cnt += 1;
            if cnt > 4 {
                return 0;
            }
            sum += i;
        }
    }
    if cnt < 4 { return 0; }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(32, Solution::sum_four_divisors(vec![21,4, 7]));
        assert_eq!(64, Solution::sum_four_divisors(vec![21,21]));

    }
}
