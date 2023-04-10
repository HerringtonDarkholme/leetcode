impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let mut start: Vec<_> = start.bytes().collect();
        let mut target: Vec<_> = target.bytes().collect();
        for left in 0..start.len() {
            if target[left] == b'L' && start[left] == b'_' {
                for i in (left+1)..start.len() {
                    if start[i] == b'L' {
                        start.swap(i, left);
                        break;
                    } else if start[i] == b'R' {
                        return false;
                    }
                }
            }
        }
        for right in (0..start.len()).rev() {
            if target[right] == b'R' && start[right] == b'_' {
                for i in (0..right).rev() {
                    if start[i] == b'R' {
                        start.swap(right, i);
                        break;
                    } else if start[i] == b'L' {
                        return false;
                    }
                }
            }
        }
        for i in 0..start.len() {
            if start[i] != target[i] {
                return false;
            }
        }
        true
    }
}
//  b
// _L__R__R__
//         a
// L_______RR
