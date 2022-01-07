impl Solution {
    pub fn maximum_invitations(fav: Vec<i32>) -> i32 {
        let mut states = vec![State::Unitialized; fav.len()];
        let mut max = 0;
        use State::*;
        for i in 0..fav.len() {
            find_loop(&fav, i, 0, &mut states);
        }
        let mut longest = vec![0; fav.len()];
        // println!("{:?}", states);

        for s in states {
            match s {
                NTR(n, i) => {
                    longest[i] = longest[i].max(n);
                },
                Orgy(g) => max = max.max(g),
                _ => {},
            }
        }
        // merge all closed loops together
        max = max.max(longest.into_iter().sum());
        max as i32
    }
}

#[derive(Copy, Clone, Debug)]
enum State {
    Orgy(usize),
    NTR(usize, usize),
    Pending(usize),
    Bachelor,
    Unitialized,
}

fn find_loop(graph: &Vec<i32>, start: usize, cnt: usize, states: &mut Vec<State>) -> State {
    use State::*;
    match states[start] {
        Pending(u) => {
            states[start] = Orgy(cnt - u);
            Orgy(cnt - u)
        },
        Unitialized => {
            let n = graph[start] as usize;
            if graph[n] as usize == start {
                states[start] = NTR(1, start);
                states[n] = NTR(1, n);
                return NTR(1, start);
            }
            states[start] = Pending(cnt);
            let nxt = find_loop(graph, n, cnt + 1, states);
            if matches!(states[start], Orgy(_)) {
                return states[start]
            }
            states[start] = match nxt {
               NTR(u, n) => NTR(u + 1, n),
               _ => Bachelor,
            };
            states[start]
        },
        a => a,
    }
    
}
