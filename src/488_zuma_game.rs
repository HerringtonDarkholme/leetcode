const R: usize = 0;
const Y: usize = 1;
const B: usize = 2;
const G: usize = 3;
const W: usize = 4;

use std::collections::HashMap;

impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut board: Vec<_> = board.chars().map(|c| match c {
            'R' => R, 'Y' => Y, 'B' => B, 'G' => G, 'W' => W,
            _ => panic!("impossible"),
        }).collect();
        let mut hands = [0; 5];
        for c in hand.chars() {
            match c {
                'R' => hands[R] += 1,
                'Y' => hands[Y] += 1,
                'B' => hands[B] += 1,
                'G' => hands[G] += 1,
                'W' => hands[W] += 1,
                _ => panic!("impossible"),
            }
        }
        let mut cache = HashMap::new();
        dfs(&mut board, &mut hands, &mut cache)
    }
}

fn count(v: &Vec<usize>) -> [usize; 5] {
    let mut c = [0; 5];
    for &i in v {
        c[i] += 1;
    }
    c
}
fn enough_balls(board: &Vec<usize>, hand: &[usize; 5]) -> bool {
    let c = count(board);
    for i in R..=W {
        if c[i] > 0 && c[i] + hand[i] < 3 {
            return false
        }
    }
    true
}
fn mk_key(board: &mut Vec<usize>, hand: &mut [usize; 5]) -> String {
    format!("{:?}{:?}", board, hand)
}

fn dfs(board: &mut Vec<usize>, hand: &mut [usize; 5], cache: &mut HashMap<String, i32>) -> i32 {
    if board.len() == 0 {
        return 0
    }
    if !enough_balls(board, hand) {
        return -1
    }
    let key = mk_key(board, hand);
    if let Some(r) = cache.get(&key) {
        return *r
    }

    let mut min = -1;
    for i in R..=W {
       if hand[i] == 0 {
           continue;
        }
        hand[i] -= 1;
        for j in 0..=board.len() {
            if j < board.len() && board[j] == i {
                continue;
            }
            let mut n_board = board.clone();
            n_board.insert(j, i);
            cleanup(&mut n_board);
            let d = dfs(&mut n_board, hand, cache);
            if d != -1 {
                if min != -1 {
                    min = min.min(d + 1);
                } else {
                    min = d + 1;
                }
            }
        } 
        hand[i] += 1;
    }
    cache.insert(key, min);
    min
}

fn cleanup(board: &mut Vec<usize>) {
    let mut i = 0;
    while i < board.len() {
        let mut j = i + 1;
        while j < board.len() && board[i] == board[j] {
            j += 1;
        }
        if j - i >= 3 {
            drop(board.drain(i..j));
            // reset
            i = 0;
            continue;
        }
        i = j;
    }
}
