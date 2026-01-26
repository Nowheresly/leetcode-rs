
pub struct Solution {}

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let n = arr.len();
        let mut arr = arr;
        arr.sort();
        let mut res = vec![];
        let mut min = i32::MAX;
        for i in 1..n {
            let a = arr[i-1];
            let b = arr[i];
            let diff = b - a;
            min = diff.min(min);
        }
        for i in 1..n {
            let a = arr[i-1];
            let b = arr[i];
            let diff = b - a;
            if diff == min {
                res.push(vec![a,b]);
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
        assert_eq!(vec![vec![1,2],vec![2,3],vec![3,4]], Solution::minimum_abs_difference(vec![4,2,1,3]));

    }
}
