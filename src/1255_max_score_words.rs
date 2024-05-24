impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let words = build_words(words);
        let mut letters = build_letters(letters);
        dfs(0, &words, &mut letters, &score)
    }
}
fn build_words(words: Vec<String>) -> Vec<Vec<usize>> {
    words.into_iter().map(|word| {
        let mut v = vec![0; 26];
        for b in word.bytes() {
            v[(b - b'a') as usize] += 1;
        }
        v
    }).collect()
}
fn build_letters(letters: Vec<char>) -> Vec<usize> {
    let mut ret = vec![0; 26];
    for c in letters {
        let i = (c as u8 - b'a') as usize;
        ret[i] += 1;
    }
    ret
}
fn dfs(i: usize, words: &[Vec<usize>], letters: &mut Vec<usize>, scores: &[i32]) -> i32 {
    if i == words.len() {
        return 0;
    }
    let mut can_take = true;
    for (idx, &cnt) in words[i].iter().enumerate() {
        can_take = can_take && (letters[idx] >= cnt);
    }
    let mut max = -1;
    if can_take {
        let mut score = 0;
        for (idx, &cnt) in words[i].iter().enumerate() {
            if cnt == 0 { continue; }
            letters[idx] -= cnt;
            score += scores[idx] * cnt as i32;
        }
        max = dfs(i + 1, words, letters, scores) + score;
        for (idx, &cnt) in words[i].iter().enumerate() {
            letters[idx] += cnt;
        }       
    }    
    max.max(dfs(i + 1, words, letters, scores))
}
