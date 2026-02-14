
pub struct Solution {}

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut pyramid = vec![vec![0.0;102]; 102];
        pyramid[0][0] = poured as f64;
        for r in 0..=query_row {
            for c in 0..=r {
                let q = (pyramid[r as usize][c as usize] - 1.0) / 2.0;

                if q > 0.0 {
                    pyramid[r as usize + 1][c as usize] += q;
                    pyramid[r as usize + 1][c as usize + 1] += q;
                }
            }
        }
        1.0f64.min(pyramid[query_row as usize][query_glass as usize])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1.0, Solution::champagne_tower(100000009, 33,17));


    }
}
