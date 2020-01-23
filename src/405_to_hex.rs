impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".into()
        }
        let mut num = num as u32;
        let mut ret = vec![];
        let map = [
          '0', '1', '2', '3',
          '4', '5', '6', '7',
          '8', '9', 'a', 'b',
          'c', 'd', 'e', 'f'
        ];
        while num > 0 {
            ret.push(map[(num % 16) as usize]);
            num /= 16;
        }
        ret.into_iter().rev().collect()
    }
}
