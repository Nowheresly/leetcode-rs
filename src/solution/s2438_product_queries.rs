
pub struct Solution {}

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers = vec![];
        for i in 0..30 {
            if (n & (1 << i)) != 0 {
                powers.push(1 << i);
            }
        }
        let mut results = vec![];
        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            let mut product: i64 = 1;
            for i in left..=right {
                product = (product * powers[i]) % MOD;
            }
            results.push(product as i32);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2,4,64], Solution::product_queries(15, vec![vec![0,1],vec![2,2],vec![0,3]]));
        assert_eq!(vec![2], Solution::product_queries(2,vec![vec![0,0]]));

    }
}
