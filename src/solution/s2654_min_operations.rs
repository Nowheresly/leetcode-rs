pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count1 = 0;

        for i in 0..n {
            if nums[i] == 1 {
                count1 += 1;
            }
        }
        if count1 > 0 {
            return (n as i32) - count1;
        }
        let mut min_op = i32::MAX;
        for l in 0..n {
            let mut curr_gcd = 0;
            for r in l..n {
                if r - l + 1 >= min_op as usize {
                    break;
                }
                curr_gcd = gcd(curr_gcd, nums[r]);
                if curr_gcd == 1 {
                    min_op = (r - l + 1) as i32;
                }
            }
        }
        if min_op == i32::MAX {
            return -1;
        }
        min_op - 1 + (n as i32) - 1
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if a < b {
        return gcd(b, a);
    }
    let mut a = a;
    let mut b = b;
    while a != 0 && b != 0 {
        let c = a % b;
        if c == 1 {
            return 1;
        }
        a = b;
        b = c;
    }
    a.max(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::min_operations(vec![2,6,3,4]));

    }
}