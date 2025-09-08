
pub struct Solution {}

impl Solution {
    pub fn max_number_of_apples(weight: Vec<i32>) -> i32 {
        let mut weight = weight.clone();
        weight.sort_by(|a, b| a.cmp(b));
        let mut sum = 0;
        let mut count = 0;
        for w in weight {
            sum += w;
            if sum > 5000 {
                break;
            }
            count += 1;
        }
        count
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::max_number_of_apples(vec![100,200,150,1000]));
        assert_eq!(5, Solution::max_number_of_apples(vec![900,950,800,1000,700,800]));

    }
}
