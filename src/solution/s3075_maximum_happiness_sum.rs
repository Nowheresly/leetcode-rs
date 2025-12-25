use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut pq = BinaryHeap::new();
        for i in 0..happiness.len() {
            if pq.len() < k as usize {
                pq.push(Reverse(happiness[i]));
                continue;
            }
            if pq.peek().unwrap().0 > happiness[i] {
                continue;
            }
            pq.pop();
            pq.push(Reverse(happiness[i]));
        }
        let mut ans = 0i64;
        let mut del = k - 1;
        let mut k = k;
        while k > 0 {
            let val = pq.pop().unwrap().0;
            let val = (val - del).max(0);
            ans += val as i64;
            del -= 1;
            k -= 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(4, Solution::maximum_happiness_sum(vec![1,2,3], 2));
        assert_eq!(5, Solution::maximum_happiness_sum(vec![2,3,4,5], 1));
    }
}
