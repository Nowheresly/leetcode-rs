pub struct Solution {}

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut last = 0;
        let mut total_free = 0;
        let mut queue: Vec<i32> = vec![];
        let mut ret = 0;
        for i in 0..start_time.len() {
            let start = start_time[i];
            let end = end_time[i];
            let free_time = start - last;
            total_free += free_time;
            queue.push(free_time);
            last = end;
            // Remove events that have ended before the current start time
            while queue.len() > (k + 1) as usize {
                total_free -= queue.remove(0);
            }
            ret = ret.max(total_free);
        }

        let free_time = event_time - last;
        total_free += free_time;
        queue.push(free_time);
        while queue.len() > (k + 1) as usize {
            total_free -= queue.remove(0);
        }
        ret = ret.max(total_free);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::max_free_time(5, 1, vec![1, 3], vec![2, 5]));
        assert_eq!(
            6,
            Solution::max_free_time(10, 1, vec![0, 2, 9], vec![1, 4, 10])
        );
    }
}
