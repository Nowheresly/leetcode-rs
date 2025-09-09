
pub struct Solution {}

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let mut dp:Vec<i64> = vec![0; (n + 1) as usize];
        dp[1] = 1;
        let mut share:i64 = 0;
        let modu:i64 = 1_000_000_007;
        for i in 2..=n {
            share = (share + dp[(i - delay).max(0) as usize]) % modu;
            if i > forget {
                share = (share - dp[(i - forget) as usize] + modu) % modu;
            }
            dp[i as usize] = share;
        }
        let mut aware = 0;
        for i in (n - forget + 1)..=n {
            aware = (aware + dp[i as usize]) % modu;
        }
        aware as i32
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(5, Solution::people_aware_of_secret(6,2,4));
        assert_eq!(6, Solution::people_aware_of_secret(4,1,3));

    }
}
