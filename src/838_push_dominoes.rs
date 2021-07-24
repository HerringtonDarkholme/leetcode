impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut doms: Vec<_> = dominoes.chars().collect();
        let mut ret = vec!['.'; doms.len()];
        let mut i = 0;
        let mut start = 0;
        while i < doms.len() {
            if doms[i] == '.' {
                i += 1;
                continue;
            }
            if doms[i] == 'L' {
                fill_left(&mut ret, start, i+1);
                i += 1;
                start = i;
                continue;    
            } 
            start = i;
            while i < doms.len() {
                if doms[i] == 'L' {
                    fill_collide(&mut ret, start, i);
                    start = i + 1;
                    break;
                }
                if doms[i] == 'R' {
                    fill_right(&mut ret, start, i);
                    start = i;
                }
                i += 1;
                
            }
            if i == doms.len() {
                fill_right(&mut ret, start, i);
            }
        }
        ret.into_iter().collect()
    }
}

fn fill_left(r: &mut Vec<char>, start: usize, end: usize) {
    for i in start..end {
        r[i] = 'L';
    }
}
fn fill_collide(r: &mut Vec<char>, mut start: usize, mut end: usize) {
    while start < end {
        r[start] = 'R';
        r[end] = 'L';
        start += 1;
        end -= 1;
    }
    if start == end {
        r[start] = '.';
    }
}
fn fill_right(r: &mut Vec<char>, start: usize, end: usize) {
    for i in start..end {
        r[i] = 'R';
    }
}
