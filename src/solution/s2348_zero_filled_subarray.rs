
pub struct Solution {}

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut res: i64 = 0;
        let mut win = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                win += 1;
                continue;
            }
            if win == 0 {
                continue;
            }
            res += cnt(win);
            win = 0;
        }
        if win > 0 {
            res += cnt(win);
        }
        res
    }
}

fn cnt(n: i32) -> i64 {
    let mut res: i64 = 1;
    for i in 2..=n {
        res += i as i64;
    }
    res
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(6, Solution::zero_filled_subarray(vec![1,3,0,0,2,0,0,4]));
        assert_eq!(9, Solution::zero_filled_subarray(vec![0,0,0,2,0,0]));
        assert_eq!(0, Solution::zero_filled_subarray(vec![2,10,2019]));

    }
}
