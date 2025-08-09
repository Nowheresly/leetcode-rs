
pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret:Vec<i32> = vec![];

        for i in 0..(nums.len()-1) {
            let val1 = nums[i];
            for j in (i+1)..nums.len() {
                let val2 = nums[j];
                if val1 + val2 == target {
                    ret.push(i as i32);
                    ret.push(j as i32);
                    return ret;
                }
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
        assert_eq!(vec![0,1], Solution::two_sum(vec![2,7,11,15],9));
        assert_eq!(vec![1,2], Solution::two_sum(vec![3,2,4],6));
        assert_eq!(vec![0,1], Solution::two_sum(vec![3,3],6));

    }
}
