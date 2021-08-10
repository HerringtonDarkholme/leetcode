impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut r = (0, 0);
        for c in s.chars() {
            let m = r.0.min(r.1);
            r = if c == '0' {
                (r.0, m + 1)
            } else {
                (r.0 + 1, m)
            }
        }
        r.0.min(r.1)
    }
}
/*
the last char in a mono-bin-str(mbs) is either 0 or 1
dp on the last char, based on 0/1
if the i-th char is 0, then the min cost to maintain a mbs is r0
if the i-th char is 1, then the min cost to maintain a mbs is r1
to have a mbs on i+1th char, if i+1 is a zero-ended mbs
then the preceding mbs must ends with 0, so r0 = r0 | r0+1
if i+1 is a one-ended mbs, the preceding mbs can be either r0 or r1

I: 00001100000
0: 00001222222
1: 12341123333
I: 00110
0: 00122
1: 11001
*/
