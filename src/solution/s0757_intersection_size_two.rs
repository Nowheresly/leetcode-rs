pub struct Solution {}

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            if a[1] == b[1] {
                b[0].cmp(&a[0])
            } else {
                a[1].cmp(&b[1])
            }
        });
        let mut res = 0;
        let mut p1 = -1;
        let mut p2 = -1;
        for i in 0..n {
            let left = intervals[i][0];
            let right = intervals[i][1];
            if p2 < left {
                res += 2;
                p1 = right - 1;
                p2 = right;
            } else if p1 < left {
                res += 1;
                p1 = p2;
                p2 = right;
            }
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
            5,
            Solution::intersection_size_two(vec![vec![1, 3], vec![3, 7], vec![8, 9]])
        );
    }
}
