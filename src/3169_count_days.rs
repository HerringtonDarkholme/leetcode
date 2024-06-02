impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
      meetings.sort_by_key(|m| m[0]);
      let mut ret = 0;
      let mut last = 0;
      for meeting in meetings {
        let start = meeting[0];
        let end = meeting[1];
        if start > last {
          ret += start - last - 1;
        }
        last = end.max(last);
      }
      ret += days - last;
      ret
    }
}
