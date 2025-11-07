
pub struct Solution {}

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len();
        let mut delta:Vec<i64> = vec![0; n+1];

        let mut smin:i64 = stations[0] as i64;
        let mut smax = k as i64;

        for i in 0..n {
            smin = smin.min(stations[i] as i64);
            smax += stations[i] as i64;
            let left = i32::max(0, i as i32 - r);
            delta[left as usize] += stations[i] as i64;
            let right = i32::min(n as i32, i as i32 + r + 1);
            delta[right as usize] -= stations[i] as i64;
        }

        let mut res = 0i64;

        while smin <= smax {
            let target = smin + (smax - smin) / 2;
            if bin_search(&delta, k, r, target) {
                res = target as i64;
                smin = target + 1;
            } else {
                smax = target - 1;
            }
        }
        res
    }
}

fn bin_search(delta: &[i64], k: i32, r: i32, target: i64) -> bool {
    let mut cur = 0i64;
    let mut delta_cpy = delta.to_vec();
    let n = delta.len() - 1;
    let mut k = k;
    for i in 0..n {
        cur += delta_cpy[i];
        if cur < target {
            let diff = target - cur;
            if diff > k as i64 {
                return false;
            }
            k -= diff as i32;
            cur += diff;
            let right = i32::min(n as i32, i as i32 + 2 * r + 1);
            delta_cpy[right as usize] -= diff;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::max_power(vec![1,2,4,5,0], 1, 2));

    }
}
