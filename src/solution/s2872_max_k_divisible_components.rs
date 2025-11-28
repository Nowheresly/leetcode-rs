use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut values = values;
        let mut adj = HashMap::new();
        for i in 0..n {
            adj.insert(i, vec![]);
            values[i as usize] = values[i as usize] % k;
        }
        for e in edges {
            adj.get_mut(&(e[0])).unwrap().push(e[1]);
            adj.get_mut(&(e[1])).unwrap().push(e[0]);
        }
        let mut queue = vec![];
        for i in 0..n as usize {
            if adj.get(&(i as i32)).unwrap().len() == 1 {
                queue.push(i);
            }
        }
        let mut ret = 0;
        while queue.len() > 0 {
            let node = queue.pop().unwrap();
            if values[node] % k == 0 {
                ret += 1;
                if adj.get_mut(&(node as i32)).unwrap().len() == 1 {
                    let other = adj.get(&(node as i32)).unwrap().first().unwrap().clone();
                    adj.get_mut(&(node as i32)).unwrap().clear();
                    adj.get_mut(&other).unwrap().retain(|&x| x != node as i32);
                    if adj.get(&other).unwrap().len() == 1 {
                        queue.push(other as usize);
                    }
                }
                continue;
            }

            let other = adj.get(&(node as i32)).unwrap().first().unwrap().clone();
            // merge
            values[other as usize] += values[node];
            values[node] = 0;
            values[other as usize] %= k;

            adj.get_mut(&(node as i32)).unwrap().clear();
            adj.get_mut(&other).unwrap().retain(|&x| x != node as i32);
            if adj.get(&other).unwrap().len() == 1 {
                queue.push(other as usize);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::max_k_divisible_components(5,vec![vec![0,2],vec![1,2],vec![1,3],vec![2,4]], vec![1,8,1,4,4], 6));

    }
}
