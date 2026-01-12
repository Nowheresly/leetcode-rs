pub struct Solution {}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;

        for i in 1..points.len() {
            res += dist(&points[i - 1], &points[i]);
        }

        res
    }
}

fn dist(p0: &Vec<i32>, p1: &Vec<i32>) -> i32 {
    let diffx = (p0[0] - p1[0]).abs();
    let diffy = (p0[1] - p1[1]).abs();
    let diag = diffy.min(diffx);
    let other = (diffx - diffy).abs();
    diag + other
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::min_time_to_visit_all_points(vec![vec![1,1],vec![3,4],vec![-1,0]]));
    }
}
