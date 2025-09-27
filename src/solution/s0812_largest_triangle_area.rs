pub struct Solution {}

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut res = 0f64;
        let n = points.len();
        for i in 0..n {
            let pi = &points[i];
            let xi = pi[0] as f64;
            let yi = pi[1] as f64;
            for j in (i + 1)..n {
                let pj = &points[j];
                let xj = pj[0] as f64;
                let yj = pj[1] as f64;

                for k in (j + 1)..n {
                    let pk = &points[k];
                    let xk = pk[0] as f64;
                    let yk = pk[1] as f64;
                    let tmp = 0.5f64 * (xi * (yj - yk) + xj * (yk - yi) + xk * (yi - yj));
                    res = res.max(tmp.abs());
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
        assert_eq!(
            2.0f64,
            Solution::largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ])
        );
    }
}
