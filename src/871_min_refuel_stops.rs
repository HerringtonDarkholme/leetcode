use std::collections::BTreeMap;
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        stations.push(vec![target, 0]);
        let mut state = BTreeMap::new();
        state.insert(0, start_fuel);
        let mut distance = 0;
        for station in stations {
            let mut cost = station[0] - distance;
            let mut next = BTreeMap::new();
            for (&c, &f) in state.iter().rev() {
                if f < cost {
                    continue;
                }
                next.insert(c, f - cost);
                let nf = f - cost + station[1];
                if let Some(&ff) = state.get(&(c+1)) {
                    if ff - cost >= nf {
                        continue;
                    }
                }
                next.insert(c + 1, nf);
            }
            state = next;
            distance = station[0];
        }

        state.into_iter().map(|c| c.0).next().unwrap_or(-1)
    }
}
/*
stops = (3, 10), (5, 20)
initial:
state = (0, 10)
first stop: 
state = (0,7), (1, 17)
second stop:
state = (0, 5), (1, 25), (2, 35)

*/
