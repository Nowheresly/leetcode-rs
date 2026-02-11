pub struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut beg = 0i64;
        let mut end = n as i64;
        let n = n as i64;

        while beg <= end  {
            let mid = beg + (end - beg) / 2;
            let cur = mid * (mid + 1) / 2;
            if cur == n {
                return mid as i32;
            }

            if n < cur {
                end = mid - 1;
            } else {
                beg = mid + 1;
            }
        }

        end as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::arrange_coins(5)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::arrange_coins(8)
        );
    }
}
