pub struct Solution {}

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut ret = 1;
        let n = nums.len();
        let mut start = 0;
        let mut queue = vec![];
        while start <n {
            let mut cnt = 1;
            while start < n - 1  && nums[start] < nums[start + 1] {
                cnt += 1;
                start += 1;
            }
            queue.push(cnt);
            start += 1;
        }

        let mut prev = queue.pop().unwrap();
        ret = ret.max(prev / 2);
        while queue.is_empty() == false {
            let cur = queue.pop().unwrap();
            ret = ret.max( i32::min(prev, cur));
            ret = ret.max(cur / 2);
            prev = cur;
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::max_increasing_subarrays(vec![2,5,7,8,9,2,3,4,3,1]));
        assert_eq!(2, Solution::max_increasing_subarrays(vec![1,2,3,4,4,4,4,5,6,7]));
    }
}