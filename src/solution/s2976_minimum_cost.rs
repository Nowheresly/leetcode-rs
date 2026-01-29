use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut adj = vec![vec![]; 26];
        for i in 0..original.len() {
            adj[original[i] as usize - 'a' as usize].push((changed[i], cost[i] as i64));
        }
        let mut shortest = vec![vec![0i64; 26]; 26];
        for c in 'a'..='z' {
            let all_costs = compute(c, &adj);
            shortest[c as usize - 'a' as usize] = all_costs;
        }
        let mut res = 0;
        for (s, t) in source.chars().zip(target.chars()) {
            let cst = shortest[s as usize - 'a' as usize][t as usize - 'a' as usize];
            if cst == -1 {
                return -1;
            }
            res += cst;
        }
        res
    }
}

fn compute(source: char, adj: &[Vec<(char, i64)>]) -> Vec<i64> {
    let mut cost = vec![i64::MAX; 26];
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0i64), source));

    while let Some((Reverse(c), u)) = pq.pop() {
        if cost[u as usize - 'a' as usize] > c {
            cost[u as usize - 'a' as usize] = c;
        }
        for (nu, nc) in adj[u as usize - 'a' as usize].iter() {
            let new_cost = c + nc;

            if cost[*nu as usize - 'a' as usize] > new_cost {
                pq.push((Reverse(new_cost), *nu));
            }
        }
    }
    for i in 0..26 {
        if cost[i] == i64::MAX {
            cost[i] = -1;
        }
    }
    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            28,
            Solution::minimum_cost(
                String::from("abcd"),
                String::from("acbe"),
                vec!['a', 'b', 'c', 'c', 'e', 'd'],
                vec!['b', 'c', 'b', 'e', 'b', 'e'],
                vec![2, 5, 5, 1, 2, 20]
            )
        );
    }
}
