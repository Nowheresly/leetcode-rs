
pub struct Solution {}

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let n = fruits.len();
        let mut left = 0;
        let mut ret = 0;
        let mut count = 0;
        let mut map = std::collections::HashMap::new();
        for right in 0..n {
            let fruit = fruits[right];
            *map.entry(fruit).or_insert(0) += 1;
            count += 1;

            while map.len() > 2 {
                let left_fruit = fruits[left];
                if let Some(&count_left) = map.get(&left_fruit) {
                    if count_left == 1 {
                        map.remove(&left_fruit);
                    } else {
                        *map.get_mut(&left_fruit).unwrap() -= 1;
                    }
                }
                left += 1;
                count -= 1;
            }

            ret = ret.max(count);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::total_fruit(vec![1,2,1]));
        assert_eq!(3, Solution::total_fruit(vec![0,1,2,2]));
        assert_eq!(4, Solution::total_fruit(vec![1,2,3,2,2]));
    }
}
