
pub struct Solution {}

impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        for i in 0..n {
            let target = mat[0][i];
            let mut good_target = true;
            for j in 1..m {
                let found = find(target, &mat[j]);
                if found == false {
                    good_target = false;
                    break;
                }
            }
            if good_target {
                return target;
            }
        }
        -1
    }
}

fn find(target: i32, num: &Vec<i32>) -> bool {
    let mut start: i32 = 0;
    let mut end: i32 = num.len() as i32 - 1;
    while start <= end {
        let mid = start + (end - start) / 2;
        if num[mid as usize] == target {
            return true;
        } else if num[mid as usize] < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::smallest_common_element(vec![vec![1,2,3,4,5],vec![2,4,5,8,10],vec![3,5,7,9,11],vec![1,3,5,7,9]]));
        assert_eq!(2, Solution::smallest_common_element(vec![vec![1,2,3],vec![2,3,4],vec![2,3,5]]));
    }
}
