use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n as usize];
        for e in edges {
            adj[e[0] as usize].push((e[1], e[2]));
            adj[e[1] as usize].push((e[0], 2 * e[2]));
        }
        let mut cost = vec![i32::MAX; n as usize];

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0i32), 0i32));

        while let Some((Reverse(node_cost), node)) = pq.pop() {
            if cost[node as usize] < node_cost {
                continue;
            }
            cost[node as usize] = node_cost;
            for &(neig_node, neig_cost) in adj[node as usize].iter() {
                if cost[neig_node as usize] != i32::MAX {
                    continue;
                }
                pq.push((Reverse(node_cost + neig_cost), neig_node));
            }
        }
        if cost[n - 1] == i32::MAX {
            return -1;
        }
        cost[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::min_cost(
                4,
                vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]]
            )
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::min_cost(
                4,
                vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]]
            )
        );
    }
}
