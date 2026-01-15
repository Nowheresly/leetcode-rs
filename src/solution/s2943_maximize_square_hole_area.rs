
pub struct Solution {}

impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let mut h_bars = h_bars;
        let mut v_bars = v_bars;
        h_bars.sort_unstable();
        v_bars.sort_unstable();

        let mut cur = 1;
        let mut hmax = 1;
        for i in 1..h_bars.len() {
            if h_bars[i-1] + 1 == h_bars[i] {
                cur += 1;
            } else {
                cur = 1;
            }
            hmax = std::cmp::max(hmax, cur);
        }
        let mut vmax = 1;
        cur = 1;
        for i in 1..v_bars.len() {
            if v_bars[i-1] + 1 == v_bars[i] {
                cur += 1;
            } else {
                cur = 1;
            }
            vmax = std::cmp::max(vmax, cur);
        }
        let side = std::cmp::min(hmax, vmax) + 1;
        side * side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::maximize_square_hole_area(2,3,vec![2,3], vec![2,4]));

    }
}
