
pub struct Solution {}

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut ret = vec![];
        let even = n % 2 == 0;
        if even {
            for i in 0..n/2 {
                ret.push(i - n / 2);
            }
            for i in n/2..n {
                ret.push(n - i);
            }
        } else {
            for i in 0..n {
                ret.push(i - n / 2);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ret = Solution::sum_zero(5);
        assert_eq!(5, ret.len());
        assert_eq!(0, ret.into_iter().sum::<i32>());
    }
}
