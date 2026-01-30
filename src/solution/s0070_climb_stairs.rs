pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut cache = vec![0; 46];

        cb(&mut cache, n)
    }
}

fn cb(cache: &mut Vec<i32>, n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    if cache[n as usize] != 0 {
        return cache[n as usize];
    }
    let a = cb(cache, n - 1);
    let b = cb(cache, n - 2);
    cache[n as usize] = a + b;
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
    }
}
