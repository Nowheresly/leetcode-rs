use std::collections::{HashMap, HashSet};

pub struct Solution {}
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut points = vec![];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    points.push((i, j));
                }
            }
        }
        let mut map = HashMap::new();
        for point in points {
            let mut set = HashSet::new();
            for i in 0..9 {
                if board[point.0][i] != '.' {
                    set.insert(board[point.0][i]);
                }
            }
            for i in 0..9 {
                if board[i][point.1] != '.' {
                    set.insert(board[i][point.1]);
                }
            }
            manage_part(&board, &point, &mut set);
            map.insert(point, set);
        }

        let _ret = dfs(board, &mut map);
    }
}

fn dfs(board: &mut Vec<Vec<char>>, map: &mut HashMap<(usize,usize), HashSet<char>>) -> bool {
    if map.len() == 0 {
        return true;
    }
    let mut score = 0;
    let mut candidate:(usize,usize) = (0,0);
    for (p, set) in map.iter() {
        let s = set.len();
        if s > score {
            score = s;
            candidate = p.clone();
        }
    }

    // ok, we have a candidate, populates the choices
    let mut choices = HashSet::new();
    let set = map.get(&candidate).unwrap();
    for c in '1'..='9' {
        if !set.contains(&c) {
            choices.insert(c);
        }
    }

    let cand_part = (candidate.0 / 3, candidate.1 / 3);
    // now dfs and backtrack for each choice
    for c in choices {
        let mut copy = map.clone();
        board[candidate.0][candidate.1] = c;
        copy.remove(&candidate);
        for (p, set) in copy.iter_mut() {
            // same row
            if p.0 == candidate.0 {
                set.insert(c);
            }
            // same column
            if p.1 == candidate.1 {
                set.insert(c);
            }
            //same part
            let part = (p.0 / 3, p.1 / 3);
            if part == cand_part {
                set.insert(c);
            }
        }
        if dfs(board, &mut copy) {
            return true;
        }
        // otherwise backtrack
        board[candidate.0][candidate.1] = '.';
    }
    false
}

fn manage_part(board: &Vec<Vec<char>>, p: &(usize,usize), found: &mut HashSet<char>) {
    if p.0 < 3 && p.1 < 3 {
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] != '.' {
                    found.insert(board[i][j]);
                }
            }
        }
        return;
    }
    if p.0 < 3 && p.1 >= 3 && p.1 < 6 {
        for i in 0..3 {
            for j in 3..6 {
                if board[i][j] != '.' {
                    found.insert(board[i][j]);
                }
            }
        }
        return;
    }
    if p.0 < 3 && p.1 >= 6 {
        for i in 0..3 {
            for j in 6..9 {
                if board[i][j] != '.' {
                    found.insert(board[i][j]);
                }
            }
        }
        return;
    }

    // second row part
    if p.0 >= 3 && p.0 < 6 && p.1 < 3 {
        for i in 3..6 {
            for j in 0..3 {
                if board[i][j] != '.' {
                    found.insert(board[i][j]);
                }
            }
        }
        return;
    }
    if p.0 >= 3 && p.0 < 6 && p.1 >= 3 && p.1 < 6 {
        for i in 3..6 {
            for j in 3..6 {
                if board[i][j] != '.' {
                    found.insert(board[i][j]);
                }
            }
        }
        return;
    }
    if p.0 >= 3 && p.0 < 6 && p.1 >= 6 {
        for i in 3..6 {
            for j in 6..9 {
                if board[i][j] != '.' {
                    found.insert(board[i][j]);
                }
            }
        }
        return;
    }

    // third part
    if p.0 >= 6 && p.1 < 3 {
        for i in 6..9 {
            for j in 0..3 {
                if board[i][j] != '.' {
                    found.insert(board[i][j]);
                }
            }
        }
        return;
    }
    if p.0 >= 6 && p.1 >= 3 && p.1 < 6 {
        for i in 6..9 {
            for j in 3..6 {
                if board[i][j] != '.' {
                    found.insert(board[i][j]);
                }
            }
        }
        return;
    }
    if p.0 >= 6 && p.1 >= 6 {
        for i in 6..9 {
            for j in 6..9 {
                if board[i][j] != '.' {
                    found.insert(board[i][j]);
                }
            }
        }
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ],
            board
        );
    }
}
