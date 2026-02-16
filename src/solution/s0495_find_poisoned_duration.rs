pub struct Solution {}

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut res = 0;
        let mut prev = time_series[0];
        for t in time_series.iter() {
            let start = prev.max(*t);
            let end = t + duration;
            res += end - start;
            prev = end;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            4,
            Solution::find_poisoned_duration(
                vec![1,4],
                2
            )
        );
    }
}
