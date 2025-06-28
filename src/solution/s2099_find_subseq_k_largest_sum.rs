pub struct Solution {}

struct A {
    val: i32,
    idx: usize,
}

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut vec: Vec<A> = nums
            .iter()
            .enumerate()
            .map(|(idx, &val)| A { val, idx })
            .collect();
        // sort by value descending
        vec.sort_by(|a, b| b.val.cmp(&a.val));

        // take the first k elements
        let mut result: Vec<&A> = vec.iter().take(k as usize).collect();

        // sort by index ascending
        result.sort_by(|a, b| a.idx.cmp(&b.idx));
        return result.iter().map(|a| a.val).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![3, 3], Solution::max_subsequence(vec![2, 1, 3, 3], 2));
        assert_eq!(
            vec![-1, 3, 4],
            Solution::max_subsequence(vec![-1, -2, 3, 4], 3)
        );
    }
}
