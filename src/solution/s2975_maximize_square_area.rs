use std::collections::HashSet;

pub struct Solution {}


impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        // Helper to get all possible distances between any two fences (including boundaries)
        let h_distances = get_all_distances(h_fences, m);
        let v_distances = get_all_distances(v_fences, n);

        let mut max_side = -1;

        // We want the largest common distance
        for side in h_distances {
            if v_distances.contains(&side) {
                max_side = max_side.max(side as i64);
            }
        }

        if max_side == -1 {
            return -1;
        }

        let MOD = 1_000_000_007;
        ((max_side * max_side) % MOD) as i32
    }
}

fn get_all_distances(mut fences: Vec<i32>, limit: i32) -> HashSet<i32> {
    let mut distances = HashSet::new();

    // Add the boundary fences
    fences.push(1);
    fences.push(limit);
    fences.sort_unstable();

    // Calculate every possible gap between any two fences
    for i in 0..fences.len() {
        for j in i + 1..fences.len() {
            distances.insert(fences[j] - fences[i]);
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::maximize_square_area(4,3,vec![2,3], vec![2]));

    }
}
