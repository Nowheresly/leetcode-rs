

pub struct Solution {}

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut max_worker_time = 0;
        for worker_time in worker_times.iter() {
            if *worker_time > max_worker_time {
                max_worker_time = *worker_time;
            }
        }
        let mut l:i64 = 1;
        let mut r:i64 = max_worker_time as i64 * mountain_height as i64 * (mountain_height as i64 + 1) /2;
        let mut ans = 0;
        let eps = 1e-7;
        while l <= r {
            let mid = (l+r)/2;
            let mut cnt = 0;
            for t in worker_times.iter() {
                let work = mid / *t as i64;
                let k = (-1.0 + f64::sqrt(1.0 + work as f64 * 8.0)) / 2.0 + eps;
                let k = k as i64;
                cnt += k;
            }

            if cnt >= mountain_height as i64 {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12, Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]));
    }
}
