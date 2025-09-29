use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {

        dp(&mut HashMap::new(), &values, 0, values.len() - 1)
    }
}

fn dp(cache: &mut HashMap<(usize, usize), i32>, values: &Vec<i32>, start: usize, end: usize) -> i32 {
    if end < start + 2 {
        return 0;
    }
    let key = (start, end);
    if let Some(v) = cache.get(&key) {
        return *v;
    }
    let mut min = i32::MAX;
    for k in (start+1)..end {

        min = min.min(values[start] * values[k] * values[end] + dp(cache, &values, start, k) + dp(cache, &values, k, end));
    }
    cache.insert(key, min);
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::min_score_triangulation(vec![1, 2, 3]));
        assert_eq!(144, Solution::min_score_triangulation(vec![3, 7, 4, 5]));
        assert_eq!(
            13,
            Solution::min_score_triangulation(vec![1, 3, 1, 4, 1, 5])
        );
    }
}
