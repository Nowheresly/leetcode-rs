
pub struct Solution {}

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut points = points.clone();
        points.sort_by(|a, b| {
           if a[0] == b[0] {
               b[1].cmp(&a[1])
           } else {
               a[0].cmp(&b[0])
            }
        });
        let n = points.len();
        for i in 0..n {
            let upper_y = points[i][1];
            let mut lower_y = i32::MIN;
            for j in i+1..n {
                let current_y = points[j][1];
                if current_y <= upper_y && current_y > lower_y {
                    res += 1;
                    lower_y = current_y;
                    if current_y == upper_y {
                        break;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::number_of_pairs(vec![vec![1,1],vec![2,2],vec![3,3]]));
        assert_eq!(2, Solution::number_of_pairs(vec![vec![6,2],vec![4,4],vec![2,6]]));
        assert_eq!(1, Solution::number_of_pairs(vec![vec![0,0],vec![0,3]]));
    }
}
