
pub struct Solution {}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let n = spells.len();
        let mut potions = potions;
        potions.sort();

        let mut result = vec![0; n];

        for i in 0..n {
            let spell = spells[i];
            result[i] = binary_search(spell, &potions, success);
        }
        result
    }
}

fn binary_search(spell: i32, potions: &Vec<i32>, success: i64) -> i32 {
    let m = potions.len() as i32;
    let mut beg:i32 = 0;
    let mut end:i32 = potions.len() as i32 -1;
    let mut ret:i32 = 0;
    while beg <= end {
        let mid = beg + (end - beg)/2;
        let mul = spell as i64 * potions[mid as usize] as i64;
        if mul >= success {
            end = mid - 1;
            ret = m - mid;
        } else {
            beg = mid + 1;
        }
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(vec![4,0,3], Solution::successful_pairs(vec![5,1,3], vec![1,2,3,4,5], 7));
        assert_eq!(vec![2,0,2], Solution::successful_pairs(vec![3,1,2], vec![8,5,8], 16));

    }
}
