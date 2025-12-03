use std::collections::HashMap;
use std::hash::{Hash, Hasher};
#[derive(Debug, Copy, Clone)]
pub struct Wrap<T>(pub T);

// Implementation for Eq
impl Eq for Wrap<f64> {}

// Implementation for Hash
impl Hash for Wrap<f64> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use the exact bit representation for hashing
        // This is safe and reliable as it converts the f64 into a u64
        self.0.to_bits().hash(state);
    }
}

// Implementation for PartialEq (required for Eq)
impl PartialEq for Wrap<f64> {
    fn eq(&self, other: &Self) -> bool {
        // We use total_cmp to handle edge cases like NaN consistently.
        // It returns Equal if the two floats have the exact same bit pattern.
        self.0.total_cmp(&other.0).is_eq()
    }
}

pub struct Solution {}

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let inf = 1_000_000_007f64;
        let mut slope_to_intercept: HashMap<Wrap<f64>, Vec<f64>> = HashMap::new();
        let mut mid_to_slope = HashMap::new();
        let mut ans = 0;

        for i in 0..n {
            let x1 = points[i][0];
            let y1 = points[i][1];
            for j in (i + 1)..n {
                let x2 = points[j][0];
                let y2 = points[j][1];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let k: f64;
                let b: f64;
                if x2 == x1 {
                    k = inf;
                    b = x1 as f64;
                } else {
                    k = dy as f64 / dx as f64;
                    // b = y1 as f64 - k * x1 as f64;
                    b = (y1 * dx - x1 * dy) as f64 / dx as f64;
                }
                let k = if k == -0.0 { 0.0 } else { k };
                let b = if b == -0.0 { 0.0 } else { b };
                slope_to_intercept.entry(Wrap(k)).or_insert(vec![]);
                slope_to_intercept.get_mut(&Wrap(k)).unwrap().push(b);

                let mid = (x1 + x2) * 10000 + (y1 + y2);
                mid_to_slope.entry(mid).or_insert(vec![]);
                mid_to_slope.get_mut(&mid).unwrap().push(k);
            }
        }
        for list in slope_to_intercept.values() {
            if list.len() == 1 {
                continue;
            }
            let mut counter = HashMap::new();
            for &b in list {
                counter.entry(Wrap(b)).or_insert(0);
                *counter.get_mut(&Wrap(b)).unwrap() += 1;
            }
            let mut sum = 0;
            for (_, v) in counter.iter() {
                ans += sum * v;
                sum += v;
            }
        }
        for list in mid_to_slope.values() {
            if list.len() == 1 {
                continue;
            }
            let mut counter = HashMap::new();
            for &k in list {
                counter.entry(Wrap(k)).or_insert(0);
                *counter.get_mut(&Wrap(k)).unwrap() += 1;
            }
            let mut sum = 0;
            for (_, v) in counter.iter() {
                ans -= sum * v;
                sum += v;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::count_trapezoids(vec![
                vec![-3, 2],
                vec![3, 0],
                vec![2, 3],
                vec![3, 2],
                vec![2, -3]
            ])
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::count_trapezoids(vec![
                vec![34, 88],
                vec![-62, -38],
                vec![26, 88],
                vec![91, 88],
                vec![47, -38]
            ])
        );
    }
}
