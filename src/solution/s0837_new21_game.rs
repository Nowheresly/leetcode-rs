use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }
        let mut window_sum = 0.0;
        for i in k..(k + max_pts) {
            if i <= n {
                window_sum += 1.0;
            }
        }
        let mut dp = HashMap::new();
        for i in (0..k ).rev() {
            dp.insert(i, window_sum / max_pts as f64);
            let mut remove = 0.0;
            if i + max_pts <= n {
                remove= *dp.get(&(i + max_pts)).unwrap_or(&1.0);
            }
            window_sum += dp.get(&i).unwrap() - remove;
        }
        (*dp.get(&0).unwrap() * 100000.0).round() / 100000.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1f64, Solution::new21_game(10, 1, 10));
        assert_eq!(0.6f64, Solution::new21_game(6, 1, 10));
        assert_eq!(0.73278, Solution::new21_game(21, 17, 10));

    }
}
