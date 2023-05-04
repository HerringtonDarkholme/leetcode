use std::collections::VecDeque;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut r_cnt = 0;
        let mut d_cnt = 0;
        let mut r_ban = 0;
        let mut d_ban = 0;
        let mut senate: VecDeque<_> = senate.bytes().collect();
        for &b in &senate {
            if b == b'R' {
                r_cnt += 1;
            } else {
                d_cnt +=1;
            }
        }
        while r_cnt > 0 && d_cnt > 0 {
            let s = senate.pop_front().unwrap();
            if s == b'R' {
                if d_ban > 0 {
                    d_ban -= 1;
                    r_cnt -= 1;
                } else {
                    r_ban += 1;
                    senate.push_back(s);
                }
            } else {
                if r_ban > 0 {
                    r_ban -= 1;
                    d_cnt -= 1;
                } else {
                    d_ban += 1;
                    senate.push_back(s);
                }
            }
        }
        if r_cnt > 0 {
            "Radiant".into()
        } else {
            "Dire".into()
        }
    }
}

// D
// R 0
// D 1
// 
