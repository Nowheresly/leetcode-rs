pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() ==1 {
            return if nums[0] == target { 0 } else { -1 };
        }
        let mut start = 0;
        let mut end = nums.len() - 1;
        loop {
            let mid = start + (end - start) / 2;
            if mid == start {
                if nums[mid] == target {
                    return mid as i32;
                }
                if nums[end] == target {
                    return end as i32;
                }
                break;
            }
            let sval = nums[start];
            let eval = nums[end];
            let mval = nums[mid];
            if target == sval {
                return start as i32;
            }
            if target == eval {
                return end as i32;
            }
            if target == mval {
                return mid as i32;
            }
            if sval < mval {
                if target > sval && target < mval {
                    end = mid;
                    continue;
                }
            }
            if mval < eval {
                if target > mval && target < eval {
                    start = mid;
                    continue;
                }
            }
            if mval > eval {
                if target > mval || target < eval {
                    start = mid;
                    continue;
                } else {
                    return -1;
                }
            }
            if sval > mval {
                if target > sval || target < mval {
                    end = mid;
                    continue;
                } else {
                    return -1;
                }
            }
            break;
        }
        1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        assert_eq!(-1, Solution::search(vec![4,5,6,7,0,1,2], 3));
        assert_eq!(-1, Solution::search(vec![1], 0));
    }
}
