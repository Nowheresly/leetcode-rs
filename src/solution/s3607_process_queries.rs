use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj = HashMap::new();
        for conn in connections.iter() {
            let u = conn[0];
            let v = conn[1];
            adj.entry(u).or_insert(Vec::new()).push(v);
            adj.entry(v).or_insert(Vec::new()).push(u);
        }

        let mut station_group: HashMap<i32, i32> = HashMap::new();
        let mut online = HashSet::new();
        let mut list = vec![BinaryHeap::new(); c as usize + 1];

        for i in 1..=c {
            dfs(i, i, &mut online, &mut station_group, &mut list, &adj);
        }

        let mut ans = Vec::new();
        for query in queries.iter() {
            let q_station = query[1];
            let q_type = query[0];
            if q_type == 2 {
                online.remove(&q_station);
                continue;
            }
            if online.contains(&q_station) {
                ans.push(q_station);
                continue;
            }
            let group_id = station_group[&q_station];
            let heap = &mut list[group_id as usize];
            let mut found = -1;
            while heap.is_empty() == false {
                let station = heap.peek().unwrap().0;
                if online.contains(&station) {
                    found = station;
                    break;
                } else {
                    heap.pop();
                }
            }
            ans.push(found);
        }
        ans
    }
}

fn dfs(
    station: i32,
    group_id: i32,
    online: &mut HashSet<i32>,
    station_group: &mut HashMap<i32, i32>,
    list: &mut Vec<BinaryHeap<Reverse<i32>>>,
    adj: &HashMap<i32, Vec<i32>>,
) {
    if online.contains(&station) {
        return;
    }
    online.insert(station);
    station_group.insert(station, group_id);
    list[group_id as usize].push(Reverse(station));
    if let Some(neighbors) = adj.get(&station) {
        for &neighbor in neighbors.iter() {
            dfs(neighbor, group_id, online, station_group, list, adj);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![3, 2, 3],
            Solution::process_queries(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
                vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]]
            )
        );
    }
}
