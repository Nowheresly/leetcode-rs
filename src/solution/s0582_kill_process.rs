use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        map.insert(0, vec![]);

        for i in 0..pid.len() {
            let val = pid[i];
            let parent = ppid[i];
            map.entry(parent).or_insert(vec![]).push(val);
            map.entry(val).or_insert(vec![]);
        }

        if kill == 0 {
            return map.keys().map(|&k| k).collect();
        }

        let mut res = vec![];

        let mut queue = vec![kill];

        while let Some(val) = queue.pop() {
            res.push(val);
            for child in map.get(&val).unwrap() {
                queue.push(*child);
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
        assert_eq!(vec![5, 10], Solution::kill_process(vec![1,3,10,5], vec![3,0,5,3], 5));
    }
}
