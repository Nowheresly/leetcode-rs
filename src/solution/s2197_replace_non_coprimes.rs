use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let mut cache = HashMap::new();

        for n in nums {
            ret.push(n);

            while ret.len() >= 2 {
                let a = ret.pop().unwrap();
                let b = ret.pop().unwrap();
                let common_div = gcd(a, b, &mut cache);

                if common_div > 1 {
                    ret.push(lcm(a, b));
                } else {
                    ret.push(b);
                    ret.push(a);
                    break;
                }
            }
        }
        ret
    }
}

fn gcd(a: i32, b:i32, cache: &mut HashMap<String, i32>) -> i32 {
    let key = format!("{}_{}", a, b);
    if cache.contains_key(&key) {
        return cache[&key];
    }
    let mut gcd = 1;
    for i in 1..=a.min(b) {
        if a % i == 0 && b % i == 0 {
            gcd = i;
        }
    }
    cache.insert(key, gcd);
    gcd
}

fn lcm(a: i32, b:i32) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    }
    let absa = a.abs();
    let absb = b.abs();
    let abs_highe = absa.max(absb);
    let abs_low = absa.min(absb);
    let mut lcm = abs_highe;
    while lcm % abs_low != 0 {
        lcm += abs_highe;
    }
    lcm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![12,7,6], Solution::replace_non_coprimes(vec![6,4,3,2,7,6,2]));
        assert_eq!(vec![2,1,1,3], Solution::replace_non_coprimes(vec![2,2,1,1,3,3,3]));
    }
}
