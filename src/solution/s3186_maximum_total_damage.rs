use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut map:HashMap<i32, i64> = HashMap::new();
        for &p in &power {
            *map.entry(p).or_insert(0) += 1;
        }
        let mut keys: Vec<i32> = map.keys().cloned().collect();
        keys.sort();
        if keys.len() == 1 {
            let count = map[&keys[0]];
            return count * (keys[0] as i64);
        }
        let mut dp = vec![0i64; keys.len()];
        dp[0] = (keys[0] as i64) * map[&keys[0]];
        dp[1] = (keys[1] as i64) * map[&keys[1]];
        if keys[1] - keys[0] > 2 {
            dp[1] += dp[0];
        }
        for i in 2..keys.len() {
            let damage = keys[i];
            let prev_damage = keys[i - 1];
            if damage - prev_damage > 2 {
                dp[i] = dp[i - 1] + (damage as i64) * map[&damage];
                dp[i] = std::cmp::max(dp[i], dp[i - 2] + (damage as i64) * (map[&damage] as i64));
            } else {
                let prop1 = dp[i - 1];
                let mut j:i32 = i as i32 - 1;
                let mut prop2;
                while j >= 0 && damage - keys[j as usize] <= 2 {
                    j -= 1;
                }
                if j < 0 {
                    prop2 = 0;
                } else {
                    prop2 = dp[j as usize];
                }
                if j > 0 {
                    prop2 = prop2.max(dp[j as usize - 1]);
                }
                if j > 1 {
                    prop2 = prop2.max(dp[j as usize - 2]);
                }
                dp[i] = std::cmp::max(prop2 + (damage as i64) * map[&damage], prop1);
            }
        }
        std::cmp::max(dp[keys.len() - 1], dp[keys.len() - 2])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::maximum_total_damage(vec![1,1,3,4]));
        assert_eq!(13, Solution::maximum_total_damage(vec![7,1,6,6]));
    }
}