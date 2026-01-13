pub struct Solution {}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut max_y = 0f64;
        let mut total_area = 0f64;
        for square in squares.iter() {
            let y = square[1] as f64;
            let l = square[2] as f64;
            let area = l * l;
            total_area += area;
            max_y = max_y.max(y + l);
        }
        let mut lo = 0f64;
        let mut hi = max_y;
        let eps = 0.00001;
        while (hi - lo).abs() > eps {
            let mid = (hi + lo) / 2f64;
            if check(mid, &squares, total_area) {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        hi
    }
}

fn check(limit_y: f64, squares: &Vec<Vec<i32>>, total_area: f64) -> bool {
    let mut area = 0f64;
    for square in squares {
        let y = square[1] as f64;
        let l = square[2] as f64;

        if y < limit_y {
            area += l * l.min(limit_y - y);
        }
    }
    area >= total_area/2f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, 1.16667 < Solution::separate_squares(vec![vec![0,0,2],vec![1,1,1]]));
        assert_eq!(true, 1.166679 > Solution::separate_squares(vec![vec![0,0,2],vec![1,1,1]]));
    }
}
