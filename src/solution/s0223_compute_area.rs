pub struct Solution {}

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let area_a = (ax2 - ax1) * (ay2 - ay1);
        let area_b = (bx2 - bx1) * (by2 - by1);

        let ret = area_a + area_b;

        // intersect == yes
        let cx1 = i32::max(ax1, bx1);
        let cx2 = i32::min(ax2, bx2);
        let x_overlap = cx2 - cx1;

        let cy1 = i32::max(ay1, by1);
        let cy2 = i32::min(ay2, by2);
        let y_overlap = cy2 - cy1;

        let mut area_of_overlap = 0;
        if x_overlap > 0 && y_overlap > 0 {
            area_of_overlap = x_overlap * y_overlap;
        }
        ret - area_of_overlap
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(45, Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
        );
    }
}
