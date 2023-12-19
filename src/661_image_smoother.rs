impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut new = vec![vec![0; img[0].len()]; img.len()];
        for r in 0..img.len() {
            for c in 0..img[0].len() {
                let mut ret = 0;
                let mut cnt = 0;
                for i in [-1, 0, 1] {
                    let nr = r as i32 + i;
                    if nr < 0 || nr >= img.len() as i32 {
                        continue;
                    }
                    for j in [-1, 0, 1] {
                        let nc = c as i32 + j;
                        if nc < 0 || nc >= img[0].len() as i32 {
                            continue;
                        }
                        ret += img[nr as usize][nc as usize];
                        cnt += 1;
                    }
                }
                new[r][c] = (ret / cnt) as i32;
            }
        }
        new
    }
}
