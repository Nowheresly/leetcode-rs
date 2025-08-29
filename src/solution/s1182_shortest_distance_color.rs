
pub struct Solution {}

impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = colors.len();
        let mut dist1 = vec![-1; n];
        let mut dist2 = vec![-1; n];
        let mut dist3 = vec![-1; n];
        let mut ans:Vec<i32> = vec![];

        if colors[0] == 1 {
            dist1[0] = 0;
        } else if colors[0] == 2 {
            dist2[0] = 0;
        } else if colors[0] == 3 {
            dist3[0] = 0;
        }

        for i in 1..n {
            if colors[i] == 1 {
                handle(i, & mut dist1, & mut dist2, & mut dist3);
            } else if colors[i] == 2 {
                handle(i, & mut dist2, & mut dist1, & mut dist3);
            } else if colors[i] == 3 {
                handle(i, & mut dist3, & mut dist1, & mut dist2);
            }
        }
        for q in queries {
            let index = q[0] as usize;
            let color = q[1];
            if color == 1 {
                ans.push(dist1[index]);
            } else if color == 2 {
                ans.push(dist2[index]);
            } else if color == 3 {
                ans.push(dist3[index]);
            }
        }
        ans
    }
}

fn handle(i: usize, dist1: &mut Vec<i32>, dist2: &mut Vec<i32>, dist3: &mut Vec<i32>) {
    dist1[i] = 0;
    let mut val = 1;
    for j in (0..i).rev() {
        if dist1[j] == -1 || dist1[j] > val {
            dist1[j] = val;
            val += 1;
        } else {
            break;
        }
    }
    if dist2[i-1] != -1 {
        dist2[i] = dist2[i-1] + 1;
    }
    if dist3[i-1] != -1 {
        dist3[i] = dist3[i-1] + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![3,0,3], Solution::shortest_distance_color(vec![1,1,2,1,3,2,2,3,3], vec![vec![1,3],vec![2,2],vec![6,1]]));
        assert_eq!(vec![-1], Solution::shortest_distance_color(vec![1,2], vec![vec![0,3]]));
    }
}
