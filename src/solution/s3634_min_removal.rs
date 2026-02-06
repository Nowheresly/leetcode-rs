pub struct Solution {}

impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        if n == 1 {
            return 0;
        }

        let mut ret = i32::MAX;
        for i in 0..(n-1) {
            let min_val = nums[i];
            let target = min_val as i64 * k as i64;
            let best = bin_search(&nums, i, target);
            if best == -1 {
                break;
            }
            ret = ret.min(i as i32 + n as i32 - 1 - best as i32);
            if i as i32 > ret {
                break;
            }
        }
        ret
    }
}

fn bin_search(nums: &Vec<i32>, start: usize, target: i64) -> i32 {
    let mut ret:i32 = -1;
    let mut begin = start;
    let mut end = nums.len() - 1;

    while  begin <= end {
        let mid = begin + (end - begin) / 2;
        if nums[mid] as i64 <= target {
            ret = mid as i32;
            begin = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_removal(vec![2,1,5], 2));
    }
}
