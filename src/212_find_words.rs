use std::collections::HashSet;
impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let trie = build_full(words);
        let mut ret = HashSet::new();
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                dfs((r, c), &mut board, &trie, &mut ret);
            }
        }
        ret.into_iter().map(|s| s.to_string()).collect()
    }
}

fn dfs<'s>((r, c): (usize, usize), board: &mut Vec<Vec<char>>, trie: &'s Trie, ret: &mut HashSet<&'s str>) {
    let i = (board[r][c] as u8 - b'a') as usize;
    let Some(t) = trie.child[i].as_ref() else { return };
    if let Some(s) = &t.done { ret.insert(s); }
    board[r][c] = '-';
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (nr, nc) = (r as i32 + dx, c as i32 + dy);
        if nr < 0 || nr >= board.len() as i32 || nc < 0 || nc >= board[0].len() as i32 { continue; }
        let (nr, nc) = (nr as usize, nc as usize);
        if board[nr][nc] == '-' { continue; }
        dfs((nr, nc), board, t, ret);
    }
    board[r][c] = (b'a' + i as u8) as char;
}

#[derive(Clone, Debug)]
struct Trie {
    done: Option<String>,
    child: Vec<Option<Trie>>,
}
impl Trie {
    fn new() -> Self {
        Self {
            done: None,
            child: vec![None; 26],
        }
    }
}
fn build_trie(root: &mut Trie, word: String) {
    let mut ret = root;
    for c in word.bytes() {
        let idx = (c - b'a') as usize;
        if ret.child[idx].is_none() {
            ret.child[idx] = Some(Trie::new());
        }
        ret = ret.child[idx].as_mut().unwrap();
    }
    ret.done = Some(word);
}
fn build_full(words: Vec<String>) -> Trie {
    let mut root = Trie::new();
    for word in words {
        build_trie(&mut root, word);
    }
    root
}
