pub struct Solution {}

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let x1 = rec1[0].max(rec2[0]);
        let y1 = rec1[1].max(rec2[1]);

        let x2 = rec1[2].min(rec2[2]);
        let y2 = rec1[3].min(rec2[3]);

        let diffx = x2 - x1;
        let diffy = y2 - y1;

        if diffx <= 0 || diffy <= 0 {
            return false;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_rectangle_overlap(vec![0,0,2,2], vec![1,1,3,3]));
        assert_eq!(false, Solution::is_rectangle_overlap(vec![0,0,1,1], vec![1,0,2,1]));
    }
}
