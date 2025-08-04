pub struct Solution {}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut placed = 0;
        let mut bask = baskets.clone();

        for fruit in &fruits {
            for i in 0..bask.len() {
                if bask[i] >= *fruit {
                    placed += 1;
                    bask[i] = 0;
                    break;
                }
            }
        }
        fruits.len() as i32 - placed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4])
        );
        assert_eq!(
            0,
            Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7])
        );
    }
}
