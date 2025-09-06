pub struct Solution {}

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut ret = 0;
        for v in queries {
            let (l, r) = (v[0] as i64, v[1] as i64);
            let total = stepsum(r) - stepsum(l - 1);
            ret += (total + 1) / 2;
        }
        ret
    }
}

fn stepsum(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut ret:i64 = 0;
    let mut base:i64 = 1;
    let mut step: i64 = 1;
    while base <= n {
        let cnt = i64::min(n, base * 4 - 1) - base +1;
        ret += cnt * step;
        base *= 4;
        step += 1;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_operations(vec![vec![1,2],vec![2,4]]));
        assert_eq!(4, Solution::min_operations(vec![vec![2,6]]));

    }
}
