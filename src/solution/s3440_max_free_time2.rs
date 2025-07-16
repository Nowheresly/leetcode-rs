pub struct Solution {}

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut q: Vec<bool> = vec![false; start_time.len()];
        let mut t1 = 0;
        let mut t2 = 0;
        let n = start_time.len();
        for i in 0..n {
            if end_time[i] - start_time[i] <= t1 {
                q[i] = true;
            }
            t1 = t1.max(start_time[i] - if i == 0 { 0} else {end_time[i - 1]} );

            if end_time[n - i - 1] - start_time[n - i - 1] <= t2 {
                q[n - i - 1] = true;
            }
            t2 = t2.max(if i == 0 {event_time} else {start_time[n - i]} - end_time[n - i - 1] );
        }
        let mut ret = 0;
        for i in 0..n {
            let left = if i == 0 { 0 } else { end_time[i - 1] };
            let right = if i == n - 1 { event_time } else { start_time[i + 1] };
            if q[i] {
                ret = ret.max(right - left);
            } else {
                ret = ret.max(right - left - (end_time[i] - start_time[i]));
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
        assert_eq!(2, Solution::max_free_time(5, vec![1, 3], vec![2, 5]));
        assert_eq!(
            7,
            Solution::max_free_time(10, vec![0, 7, 9], vec![1, 8, 10])
        );
    }
}
