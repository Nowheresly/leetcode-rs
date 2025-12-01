
pub struct Solution {}

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut sum_power = 0;
        let mut min_power = batteries[0];
        for b in &batteries {
            sum_power += *b as i64;
            min_power = min_power.min(*b);
        }

        let mut left = min_power as i64;
        let mut right = sum_power / n as i64;

        while left < right {
            let mid = right - (right - left) / 2;
            let mut extra = 0;
            for b in &batteries {
                extra += (*b as i64).min(mid);
            }
            if extra >= n as i64 * mid {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::max_run_time(2, vec![3,3,3]));

    }
}
