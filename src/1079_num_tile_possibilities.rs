impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
       let mut ts = vec![0; 26];
        for c in tiles.bytes() {
            ts[(c - b'A') as usize] += 1;
        }
        num(&mut ts) - 1
    }
}

fn num(tiles: &mut [usize]) -> i32 {
    let mut ret = 1;
    for k in 0..26 {
        if tiles[k] == 0 {
            continue;
        }
        tiles[k] -= 1;
        ret += num(tiles);
        tiles[k] += 1;
    }
    ret
}
