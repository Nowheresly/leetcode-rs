pub struct Solution {}

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut f = vec![0; 3];
        f[1] = i32::MIN;
        f[2] = i32::MIN;

        for num in nums {
            let mut g = f.clone();
            for i in 0usize..3 {
                g[(i + (num as usize % 3)) % 3] =
                    i32::max(g[(i + (num as usize % 3)) % 3], f[i] + num);
            }
            f = g;
        }
        f[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(18, Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]));
    }
}
